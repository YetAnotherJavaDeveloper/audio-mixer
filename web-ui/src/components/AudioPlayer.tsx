import React, { useCallback, useImperativeHandle, useRef, useEffect, useState, useMemo } from "react";
import { Button } from "./ui/button";
import { Play, Pause, Undo2, Redo2 } from "lucide-react";
import { Slider } from "./ui/slider";
import { DeformCanvas } from "./DeformCanvas";
import { DateTime } from "luxon";
import { isNumber, min, round } from "radashi";
import { audioUtils } from "@/lib/audio/audio.utils";
import { Card, CardContent, CardFooter } from "./ui/card";
import AudioFileInput, { type FileInputRef } from "./AudioFileInput";
import { initAndPlayWasmWorklet } from "@/lib/audio/initAndPlayWasmWorklet"
import { type InitOutput, AudioBuffers } from "wasm-api";
import BarChartComponent, { type BarChartRef, type FftDataType } from "./charts/BarChart";
import type { ChartConfig } from "./ui/chart";


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

    const engineRef = useRef<AudioBuffers>(null);
    const wasmRef = useRef<InitOutput>(null);
    const chartRef = useRef<BarChartRef>(null);


    const [urlQueue, setUrlQueue] = useState<string[]>([]);

    const [currentBuffer, setCurrentBuffer] = useState<Float32Array | null>(null);
    const [duration, setDuration] = useState<number | undefined>(undefined);
    const [volume, setVolume] = useState(1);
    const [currentTime, setCurrentTime] = useState(0);
    const [isPlaying, setIsPlaying] = useState(false);
    const [currentSampleInfo, setCurrentSampleInfo] = useState<{ chanSize: number, rate: number } | undefined>(undefined);
    const [currentFrame, setCurrentFrame] = useState(0);

    const [currentMagnitudes, setCurrentMagnitudes] = useState<number[]>(new Array(2001).fill(0));
    const [fftData, setFftData] = useState<FftDataType[]>([]);
    const [inputData, setInputData] = useState<{ time: number; value: number }[]>([]);

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
      // setCurrentBuffer(new Uint8Array(arrayBuffer));
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

          wasmRef.current = worklet.wasm;
          engineRef.current = worklet.buffers;

          setDuration(round(audioBuff.duration * 1000)); // Convert to milliseconds
          setCurrentSampleInfo({
            chanSize: audioBuff.length,
            rate: audioBuff.sampleRate,
          })
          audioNodeRef.current = worklet.node;

          audioNodeRef.current.port.onmessage = (event) => {
            if (event.data.currentFrame !== undefined) {
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

    useEffect(() => {
      if (!engineRef.current || !wasmRef.current) return;
      // store once the input buffer from shared memory
      if (currentBuffer) return; // already set
      const inputBuffer = new Float32Array(wasmRef.current.memory.buffer, engineRef.current.input_ptr(), engineRef.current.len() * engineRef.current.channels());
      setCurrentBuffer(inputBuffer);
      const rate = engineRef.current.engine_state.track_config.sample_rate;
      const duration = engineRef.current.len() / rate;
      const dataForChart = [] as { time: number; value: number }[];
      inputBuffer.forEach((value, index) => {
        dataForChart.push({
          time: index / rate,
          value: value,
        });
      });

      console.log("dataForChart length:", dataForChart.length, "duration:", duration, "rate:", rate);
      setInputData(dataForChart);
      if (chartRef.current) {
        chartRef.current.setData(
          dataForChart.slice(0, 100)
            .map((d) => ({
              freq: d.time,
              magn: Math.min(1, Math.max(0, d.value)) // Scale to 0-1 range
            }))
        );
      }
    }, [currentBuffer, isPlaying]);

    useEffect(() => {
      if (!engineRef.current || !wasmRef.current) return;

      if (!engineRef.current.engine_state.is_playing) return;

      // if (currentFrame % 10 !== 0) return;

      engineRef.current.set_current_frame(currentFrame);
      engineRef.current.compute_fft();
      if (!engineRef.current) return;
      if (!engineRef.current.engine_state.fft_initialized) return;

      // read the FFT data
      const fftDataBuffer = wasmRef.current?.memory.buffer.slice(
        engineRef.current.fft_ptr,
        engineRef.current.fft_ptr + engineRef.current.fft_size * Float32Array.BYTES_PER_ELEMENT
      );
      if (fftDataBuffer) {
        const fftArray = new Float32Array(fftDataBuffer);
        // get odd values (magn part)
        const magnitudes = [];
        const fftData: FftDataType[] = [];
        fftArray.forEach((value, index) => {
          if (index % 2 === 1) {
            if (index > 0 && fftArray[index - 1] === 0) {
              return;
            }
            const bandValue = Math.min(1, Math.max(0, value / 100));
            const freq = fftArray[index - 1]; // Frequency value
            fftData.push({ freq: freq, magn: value });
          }
        });
        for (let i = 1; i < 65; i += 2) {
          // scale the magnitude to a range of 0-1
          const bandValue = Math.min(1, Math.max(0, fftArray[i] / 100));
          magnitudes.push(bandValue);
        }
        if (chartRef.current) {
          chartRef.current.setData(fftData);
        }
        setFftData(fftData);
        setCurrentMagnitudes(magnitudes);
      } else {
        console.warn("No FFT data available");
      }

    }, [currentFrame])

    return (
      <Card className="w-full h-full flex flex-col bg-white dark:bg-gray-800 shadow-md rounded-lg">

        <CardContent className="flex-grow flex flex-col justify-center relative pt-0">
          <AudioFileInput
            className="absolute top-4 right-4 z-10"
            ref={inputRef}
            onFileSelected={handleFileChange}
          />
          <div className="flex items-center justify-between mb-4">
            <DeformCanvas audioLevel={level} magnitudes={currentMagnitudes} />
          </div>
          <BarChartComponent
            className="max-h-[500px]"
            ref={chartRef}
            data={[]}
            chartConfig={{
              magn: {
                label: "Magnitude",
                color: "green",
              }
            } satisfies ChartConfig
            } />
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