import React, { useCallback, useImperativeHandle, useRef, useEffect, useState, useMemo } from "react";
import { Button } from "./ui/button";
import { Play, Pause, Undo2, Redo2 } from "lucide-react";
import { Slider } from "./ui/slider";
import { DeformCanvas } from "./DeformCanvas";
import { DateTime } from "luxon";
import { isNumber, round } from "radashi";
import { audioUtils } from "@/lib/audio/audio.utils";

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

    const [level, setLevel] = useState(0); // Simulated audio level for DeformCanvas

    useEffect(() => {
      console.debug("Duration updated:", duration);
    }, [duration]);

    const isPlayerReady = useMemo(() => {
      return audioElRef.current !== null && urlQueue.length > 0 && duration !== undefined;
    }, [duration, urlQueue.length]);

    const loadAudioFile = useCallback(async (file: File) => {
      const metadata = await audioUtils.extractAudioMetadata(file);
      console.debug("Audio metadata:", metadata);
      if (isNumber(metadata.duration) === false) {
        console.error("Invalid audio duration:", metadata.duration);
        return;
      }
      setDuration(round(metadata.duration));
      const arrayBuffer = await file.arrayBuffer();
      setCurrentBuffer(new Uint8Array(arrayBuffer));
      const blob = new Blob([file], { type: "audio/mpeg" });
      const url = URL.createObjectURL(blob);
      setUrlQueue([url]);
    }, []);

    const playAudio = useCallback(() => {
      if (audioElRef.current) {
        audioElRef.current.play().catch(console.error);
        setIsPlaying(true);
        onPlay?.();
      }
    }, [onPlay]);

    const pauseAudio = useCallback(() => {
      if (audioElRef.current) {
        audioElRef.current.pause();
        setIsPlaying(false);
        onPause?.();
      }
    }, [onPause]);

    const togglePlay = () => {
      if (isPlaying) {
        pauseAudio();
      } else {
        playAudio();
      }
    };

    const handlePositionChange = useCallback((value: number) => {
      console.debug("Setting position to:", value);
      setCurrentTime(value);
      if (audioElRef.current) {
        audioElRef.current.currentTime = round(value);
      }
      onPositionChange?.(value);
    },
      [onPositionChange]
    );

    const handleJump = useCallback((ms: number) => {
      if (!isPlayerReady) return;

      const newTime = Math.min(Math.max(0, currentTime + ms), duration!);
      handlePositionChange(newTime);
    }, [currentTime, duration, handlePositionChange, isPlayerReady]);

    const handleVolumeChange = useCallback((value: number) => {
      if (audioElRef.current) {
        audioElRef.current.volume = value;
        setVolume(value);
      }
    }, []);

    const handleTimeUpdate = useCallback(() => {
      console.debug("Time update:", audioElRef.current?.currentTime);
      const newTime = audioElRef.current!.currentTime;
      setCurrentTime(newTime);
      onPositionChange?.(newTime);
    }, [onPositionChange]);

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
      <div className="w-full max-w-md mx-auto p-4 bg-white dark:bg-gray-800 rounded-lg shadow-md">
        <div className="flex items-center justify-between mb-4">
          <DeformCanvas audioLevel={level} />
        </div>

        <div className="flex flex-col items-center">
          <Slider
            ref={playerRef}
            value={[currentTime]}
            step={0.1}
            max={duration}
            min={0}
            className="w-full h-2 bg-gray-200 dark:bg-gray-700 rounded-full"
            style={{ cursor: "pointer" }}
            onValueChange={(e) => handlePositionChange(e[0])}
          />
        </div>

        <div className="flex justify-between items-center text-sm text-gray-500 dark:text-gray-400 mt-4 space-x-4">
          <span>{isPlayerReady ? DateTime.fromMillis(currentTime * 1000).toFormat("HH:mm:ss") : "-:-:-"}</span>
          <div className="flex items-center space-x-2">
            <Button size="sm" onClick={() => handleJump(-10)}>
              <Undo2 className="h-4 w-4" />
            </Button>
            <Button size="sm" onClick={togglePlay}>
              {isPlaying ? <Pause className="h-4 w-4" /> : <Play className="h-4 w-4" />}
            </Button>
            <Button size="sm" onClick={() => handleJump(10)}>
              <Redo2 className="h-4 w-4" />
            </Button>
          </div>
          <div className="flex items-center space-x-2">
            <Slider
              value={[volume]}
              onValueChange={(e) => handleVolumeChange(e[0])}
              className="w-24 h-2 bg-gray-200 dark:bg-gray-700 rounded-full"
              step={0.01}
              max={1}
              min={0}
            />
          </div>
          <span>{isPlayerReady ? DateTime.fromMillis(duration! * 1000).toFormat("HH:mm:ss") : "-:-:-"}</span>
        </div>
      </div>
    );
  }
);

export default PlayerComponent;
