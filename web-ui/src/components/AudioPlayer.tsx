import React, { useCallback, useImperativeHandle, useRef, useEffect, useState, useMemo } from "react";
import { Button } from "./ui/button";
import { Play, Pause, Undo2, Redo2 } from "lucide-react";
import { Slider } from "./ui/slider";
import { DeformCanvas } from "./DeformCanvas";
import { DateTime } from "luxon";
import { isNumber, round } from "radashi";
import { audioUtils } from "@/lib/audio/audio.utils";
import { Card, CardContent, CardFooter } from "./ui/card";
import AudioFileInput, { type FileInputRef } from "./FileInput";
import { initAndPlayWasmWorklet } from "@/lib/audio/initAndPlayWasmWorklet";

const DEFAULT_TIME_JUMP = 5; // seconds

export interface PlayerProps {
  onPositionChange?: (_position: number) => void;
  onPlay?: () => void;
  onPause?: () => void;
}

export interface PlayerRef {
  setPosition: (_position: number) => void;
  loadAudioFile: (_file: File) => Promise<void>;
  play: () => void;
  pause: () => void;
  togglePlay: () => void;
  getState: () => {
    currentTime: number;
    isPlaying: boolean;
  };
}

const PlayerComponent = React.forwardRef<PlayerRef, PlayerProps>(
  ({ onPause, onPlay, onPositionChange }, ref) => {
    const playerRef = useRef<HTMLDivElement>(null);
    const audioElRef = useRef<HTMLAudioElement | null>(null);

    const [urlQueue, setUrlQueue] = useState<string[]>([]);

    const [currentBuffer, setCurrentBuffer] = useState<Uint8Array | null>(null);
    const [duration, setDuration] = useState<number | undefined>(undefined);
    const [volume, setVolume] = useState(1);
    const [currentTime, setCurrentTime] = useState(0);
    const [isPlaying, setIsPlaying] = useState(false);
    const [currentSampleInfo, setCurrentSampleInfo] = useState<{ chanSize: number, rate: number } | undefined>(undefined);
    const [currentFrame, setCurrentFrame] = useState(0);

    const inputRef = useRef<FileInputRef>(null);
    const audioNodeRef = useRef<AudioWorkletNode | null>(null);

    const audioContext = useMemo(() => {
      return new AudioContext();
    }, []);

    const [level, setLevel] = useState(0); // Simulated audio level for DeformCanvas

    const isPlayerReady = useMemo(() => {
      return duration !== undefined;
    }, [duration]);


    const loadAudioFile = useCallback(async (file: File) => {
      const metadata = await audioUtils.extractAudioMetadata(file);
      console.debug("Audio metadata:", metadata);
      if (isNumber(metadata.duration) === false) {
        console.error("Invalid audio duration:", metadata.duration);
        return;
      }
      const d = round(metadata.duration * 1000); // Convert to milliseconds
      console.log("Audio duration in seconds:", d);
      setDuration(d);

      console.debug(DateTime.fromMillis(metadata.duration * 1000).toFormat("HH:mm:ss"));

      const arrayBuffer = await file.arrayBuffer();
      setCurrentBuffer(new Uint8Array(arrayBuffer));
      const blob = new Blob([file], { type: "audio/mpeg" });
      const url = URL.createObjectURL(blob);
      setUrlQueue([url]);
    }, []);

    const playAudio = useCallback(() => {
      // if (audioElRef.current) {
      //   audioElRef.current.play().catch(console.error);
      //   setIsPlaying(true);
      //   onPlay?.();
      // }
      // if (audioContext.state === "suspended") {
      audioContext.resume().catch(console.error);
      setIsPlaying(true);
      onPlay?.();
      // }
    }, [onPlay, audioContext]);

    const pauseAudio = useCallback(() => {
      // if (audioElRef.current) {
      //   audioElRef.current.pause();
      //   setIsPlaying(false);
      //   onPause?.();
      // }
      audioContext.suspend().catch(console.error);
      setIsPlaying(false);
      onPause?.();
    }, [audioContext, onPause]);

    const togglePlay = () => {
      if (isPlaying) {
        pauseAudio();
      } else {
        playAudio();
      }
    };

    const handlePositionChange = useCallback((value: number) => {
      console.debug("Setting position to:", value);
      setCurrentTime(value); // Convert to milliseconds
      if (audioElRef.current) {
        audioElRef.current.currentTime = round(value);
      }
      onPositionChange?.(value);
    },
      [onPositionChange]
    );

    const seekTo = useCallback((delta: number) => {
      if (!audioNodeRef.current || !currentSampleInfo) return;
      const { rate } = currentSampleInfo;
      const node = audioNodeRef.current;
      if (!node) return;
      const targetFrame = currentFrame + round(delta * rate);

      node.port.postMessage({ seek: targetFrame });
    }, [currentSampleInfo, currentFrame]);

    const handleVolumeChange = useCallback((value: number) => {
      if (audioElRef.current) {
        audioElRef.current.volume = value;
        setVolume(value);
      }
    }, []);

    const handleTimeUpdate = useCallback(() => {
      console.debug("Time update:", audioElRef.current?.currentTime);
      const newTime = audioElRef.current!.currentTime;
      setCurrentTime(newTime * 1000); // Convert to milliseconds
      onPositionChange?.(newTime);
    }, [onPositionChange]);


    const handleFileChange = useCallback(async (file: File) => {
      if (inputRef.current) {
        inputRef.current.reset();
      }

      if (playerRef.current) {
        // loadAudioFile(file);
        try {
          const arrayBuffer = await file.arrayBuffer();
          const view = new DataView(arrayBuffer);
          const cloneBuffer = new Float32Array(arrayBuffer, view.byteOffset, view.byteLength / Float32Array.BYTES_PER_ELEMENT);

          // setArrayBuffer(new Uint8Array(arrayBuffer));
          const audioBuff = await audioContext.decodeAudioData(arrayBuffer);
          console.log('Buffer length:', arrayBuffer.byteLength);
          console.log('Float32Array length:', cloneBuffer);
          console.log('Audio buffer duration:', audioBuff.duration);
          console.log('Audio buffer sample rate:', audioBuff.sampleRate);
          console.log('Audio buffer number of channels:', audioBuff.numberOfChannels);

          const chan_0 = audioBuff.getChannelData(0);
          const chan_1 = audioBuff.getChannelData(1);

          const samplesArray = [] as Float32Array[];

          samplesArray.push(new Float32Array(chan_0));
          samplesArray.push(new Float32Array(chan_1));

          const worklet = await initAndPlayWasmWorklet(
            audioContext,
            samplesArray.length,
            samplesArray,
            audioBuff.sampleRate
          );

          setDuration(round(audioBuff.duration * 1000)); // Convert to milliseconds
          setCurrentSampleInfo({
            chanSize: audioBuff.length,
            rate: audioBuff.sampleRate,
          })
          audioNodeRef.current = worklet.node;

          audioNodeRef.current.port.onmessage = (event) => {
            if (event.data.currentFrame !== undefined) {
              console.debug("Current frame:", event.data.currentFrame);
              setCurrentFrame(event.data.currentFrame);
              setCurrentTime((event.data.currentFrame / audioBuff.sampleRate) * 1000); // Convert to milliseconds
            }
          };

        } catch (error) {
          console.error('Error reading file:', error);
        }
      }
    }, [audioContext]);


    useEffect(() => {
      if (!currentBuffer) return;

      const blob = new Blob([currentBuffer], { type: "audio/mpeg" });
      const url = window.URL.createObjectURL(blob);
      setUrlQueue((prev) => [...prev, url]);

      return () => {
        window.URL.revokeObjectURL(url);
      };
    }, [currentBuffer]);

    useEffect(() => {
      if (urlQueue.length === 0) return;

      if (!audioElRef.current) {
        const audio = new Audio();
        audio.autoplay = false;
        audio.preload = "auto";

        audio.onended = () => {
          setIsPlaying(false);
          setUrlQueue((prevQ) => prevQ.slice(1));
        };

        audioElRef.current = audio;
      }

      audioElRef.current.src = urlQueue[0];
      audioElRef.current.load();

      if (isPlaying) {
        audioElRef.current.play().catch(console.error);
      }
    }, [urlQueue, isPlaying]);

    useEffect(() => {
      if (!audioElRef.current) return;

      audioElRef.current.addEventListener("timeupdate", handleTimeUpdate);

      return () => {
        audioElRef.current?.removeEventListener("timeupdate", handleTimeUpdate);
      };
    }, [handleTimeUpdate]);

    useEffect(() => {
      const interval = setInterval(() => {
        setLevel(Math.abs(Math.sin(Date.now() / 500)));
      }, 50);
      return () => clearInterval(interval);
    }, []);

    useImperativeHandle(ref, () => ({
      setPosition: (time: number) => {
        handlePositionChange(time);
      },
      loadAudioFile,
      play: playAudio,
      pause: pauseAudio,
      togglePlay,
      getState: () => ({
        currentTime,
        isPlaying,
      }),
    }));

    return (
      <Card className="w-full h-full flex flex-col bg-white dark:bg-gray-800 shadow-md rounded-lg">

        <CardContent className="flex-grow flex flex-col justify-center relative pt-0">
          <AudioFileInput
            className="absolute top-4 right-4 z-10"
            ref={inputRef}
            onFileSelected={handleFileChange}
          />
          <DeformCanvas audioLevel={level} />
        </CardContent>

        <CardFooter className="flex flex-col w-full space-y-4 pt-4">
          {/* Timeline + timers */}
          <div className="w-full">
            <Slider
              ref={playerRef}
              value={[currentTime]}
              step={100}
              max={duration}
              min={0}
              className="w-full h-2 bg-gray-200 dark:bg-gray-700 rounded-full cursor-pointer"
              onValueChange={(e) => handlePositionChange(e[0])}
            />
            <div className="flex justify-between text-sm text-gray-500 dark:text-gray-400 mt-2">
              <span>{isPlayerReady ? audioUtils.formatMilliseconds(currentTime) : "-:-:-"}</span>
              <span>{isPlayerReady ? audioUtils.formatMilliseconds(duration!) : "-:-:-"}</span>
            </div>
          </div>

          {/* Controls + volume */}
          <div className="flex w-full items-center justify-between">
            {/* Empty space */}
            <div className="w-24" />

            {/* Player controls */}
            <div className="flex items-center space-x-2">
              <Button size="sm" onClick={() => seekTo(-DEFAULT_TIME_JUMP)}>
                <Undo2 className="h-4 w-4" />
              </Button>
              <Button size="sm" onClick={togglePlay}>
                {isPlaying ? <Pause className="h-4 w-4" /> : <Play className="h-4 w-4" />}
              </Button>
              <Button size="sm" onClick={() => seekTo(DEFAULT_TIME_JUMP)}>
                <Redo2 className="h-4 w-4" />
              </Button>
            </div>

            {/* Player volume */}
            <Slider
              value={[volume]}
              onValueChange={(e) => handleVolumeChange(e[0])}
              className="w-24 h-2 bg-gray-200 dark:bg-gray-700 rounded-full"
              step={0.01}
              max={1}
              min={0}
            />
          </div>
        </CardFooter>

      </Card>
    );
  });

export default PlayerComponent;