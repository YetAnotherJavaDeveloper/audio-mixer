import { use, useCallback, useEffect, useRef, useState } from 'react';

import init, { Engine, type InitInput, type InitOutput, RingBuffer } from 'wasm-api';

const DEFAULT_SAMPLE_RATE = 44100;
const DEFAULT_BLOCK_SIZE = 128;
const DEFAULT_OUT_CHANNELS = 2;

export interface AudioEngineProps {
  sampleRate: number; // Sample rate of the audio context
  blockSize: number; // Size of the audio block to process
  outChannels: number; // Number of output channels
}

export const useAudioEngineHook = (
  sampleRate = DEFAULT_SAMPLE_RATE,
  blockSize = DEFAULT_BLOCK_SIZE,
  outChannels = DEFAULT_OUT_CHANNELS
) => {
  // const [isInitialized, setIsInitialized] = useState(false);

  const nodeRef = useRef<AudioWorkletNode | null>(null);
  const audioCtxRef = useRef<AudioContext | null>(null);
  const engineRef = useRef<Engine | null>(null);
  const wasmRef = useRef<InitOutput>(null);

  const [trackIds, setTrackIds] = useState<number[]>([]);

  const checkEngineReady = useCallback((): boolean => {
    if (!audioCtxRef.current || !nodeRef.current) {
      console.warn('Audio engine not initialized or context/node not available');
      // console.warn('isInitialized:', isInitialized);
      console.warn('audioCtxRef:', audioCtxRef.current);
      console.warn('nodeRef:', nodeRef.current);
      return false;
    }
    return true;
  }, []);

  const addTrack = useCallback(
    async (trackBuffer: ArrayBuffer) => {
      if (!checkEngineReady()) {
        return;
      }

      const buffer: AudioBuffer = await audioCtxRef.current!.decodeAudioData(trackBuffer);

      const engine = engineRef.current!;
      const trackId = engine.add_track(buffer.length, buffer.numberOfChannels);

      const chunkSize = 64 * 1024; // 64k samples per chunk
      const totalSamples = buffer.length * buffer.numberOfChannels;

      const interleavedChunk = new Float32Array(chunkSize);

      let written = 0;
      while (written < totalSamples) {
        const remaining = totalSamples - written;
        const currentChunkSize = Math.min(chunkSize, remaining);

        // Interleave the chunk
        const startFrame = Math.floor(written / buffer.numberOfChannels);
        const framesInChunk = Math.floor(currentChunkSize / buffer.numberOfChannels);

        for (let i = 0; i < framesInChunk; i++) {
          for (let channel = 0; channel < buffer.numberOfChannels; channel++) {
            interleavedChunk[i * buffer.numberOfChannels + channel] = buffer.getChannelData(channel)[startFrame + i];
          }
        }

        // If the chunk is smaller than the full size, slice it
        const chunkToSend =
          currentChunkSize < chunkSize ? interleavedChunk.subarray(0, currentChunkSize) : interleavedChunk;

        // Write the chunk to its dedicated buffer in the engine
        engine.write_to_track(trackId, chunkToSend, written);

        written += currentChunkSize;

        // Wait for the engine to process the chunk
        // This is a simple way to ensure we don't overwhelm the engine
        await new Promise((resolve) => setTimeout(resolve));
      }

      setTrackIds((prev) => [...prev, trackId]);
      console.log(`Track ${trackId} loaded (${totalSamples} samples)`);
    },
    [checkEngineReady]
  );

  const removeTrack = useCallback(
    (trackId: number) => {
      if (!checkEngineReady()) {
        return;
      }

      const engine = engineRef.current!;
      engine.remove_track(trackId);
      setTrackIds((prev) => prev.filter((id) => id !== trackId));
    },
    [checkEngineReady]
  );

  const play = useCallback(async () => {
    if (!checkEngineReady()) {
      return;
    }

    const ctx = audioCtxRef.current!;
    const node = nodeRef.current!;

    await ctx.resume();
    node.port.postMessage({ play: true });
    engineRef.current!.play();
    console.info('Audio engine playing');
  }, [checkEngineReady]);

  const pause = useCallback(async () => {
    if (!checkEngineReady()) {
      return;
    }

    const ctx = audioCtxRef.current!;
    const node = nodeRef.current!;

    await ctx.suspend();
    node.port.postMessage({ play: false });
    engineRef.current!.pause();
    console.info('Audio engine paused');
  }, [checkEngineReady]);

  const seek = useCallback(
    (frame: number) => {
      if (!checkEngineReady()) {
        return;
      }

      const node = nodeRef.current!;
      node.port.postMessage({ seek: frame });
      console.info(`Audio engine seek command sent to frame ${frame}`);
    },
    [checkEngineReady]
  );

  const isPlaying = useCallback((): boolean => {
    if (!checkEngineReady()) {
      return false;
    }
    return engineRef.current!.is_playing();
  }, [checkEngineReady]);

  const getPlayhead = useCallback((): number => {
    if (!checkEngineReady()) {
      return 0;
    }
    return engineRef.current!.playhead;
  }, [checkEngineReady]);

  const handleWorkletMessage = useCallback(
    (e: MessageEvent) => {
      if (e.data.request === 'nextBlock') {
        if (!checkEngineReady()) return;
        // Process the next block in the engine
        engineRef.current!.process_block();
      }
    },
    [checkEngineReady]
  );

  const startEngine = useCallback(async () => {
    if (checkEngineReady()) {
      console.warn('Audio engine is already initialized');
      return;
    }

    console.info('Starting audio engine...');
    // setIsInitialized(true);

    const ctx = new AudioContext({ sampleRate });
    audioCtxRef.current = ctx;

    const wasm = await init(); // ton init wasm
    wasmRef.current = wasm;

    await ctx.audioWorklet.addModule('/worklet/wasm-engine-processor_v2.js');

    const ringBuffer = new RingBuffer(blockSize * outChannels * 4); // 4 blocks of size blockSize * outChannels

    // for (let i = 0; i < ringBuffer.len(); i++) {
    //   ringBuffer.push(0.2); // Initialize the ring buffer with zeros
    // }

    const engine = new Engine(sampleRate, outChannels, blockSize, ringBuffer);
    engineRef.current = engine;

    const sab = ringBuffer.sab; // SharedArrayBuffer directement exposé

    const wasmBuffer = new Float32Array(sab); // vue JS sur le buffer

    const node = new AudioWorkletNode(ctx, 'wasm-engine-processor_v2', {
      processorOptions: {
        ringBuffer: ringBuffer
        blockSize,
        outChannels,
      },
    });
    nodeRef.current = node;

    node.port.onmessage = handleWorkletMessage;

    node.connect(ctx.destination);

    console.info('Audio engine initialized');
  }, [checkEngineReady, sampleRate, blockSize, outChannels, handleWorkletMessage]);

  const stopEngine = useCallback(async () => {
    const ctx = audioCtxRef.current;
    const node = nodeRef.current;

    if (!ctx || !node) return;

    node.disconnect();
    await ctx.close();

    audioCtxRef.current = null;
    nodeRef.current = null;
    engineRef.current = null;
    // setIsInitialized(false);

    console.info('Audio engine stopped');
  }, []);

  // useEffect(() => {
  //   // Test every 1 second if the engine ring buffer has been filled
  //   const interval = setInterval(() => {
  //     if (!checkEngineReady()) return;

  //     const engine = engineRef.current!;
  //     console.log('[hook] OUPUT POINTER: ', engine.ring_buffer_ptr);
  //     if (wasmRef.current?.memory) {
  //       const data = new Float32Array(wasmRef.current.memory.buffer, engine.ring_buffer_ptr, engine.ring_buffer_len);
  //       // console.info(`Ring buffer read index: ${engine.ring_buffer_read_idx}`);
  //       console.info(`Read data: ${data}`);
  //     }
  //   }, 1000); // Log every second

  //   return () => clearInterval(interval);
  // }, [checkEngineReady]);

  // useEffect(() => {
  //   let ctx: AudioContext;
  //   let node: AudioWorkletNode;

  //   const initEngine = async () => {
  //     ctx = new AudioContext({ sampleRate });
  //     audioCtxRef.current = ctx;
  //     const wasm = await init();

  //     await ctx.audioWorklet.addModule('/worklet/wasm-engine-processor.js');

  //     const engine = new Engine(sampleRate, blockSize, outChannels);
  //     engineRef.current = engine;

  //     node = new AudioWorkletNode(ctx, 'wasm-engine-processor', {
  //       processorOptions: {
  //         memory: wasm.memory.buffer,
  //         outPtr: engine.output_ptr,
  //         blockSize,
  //         outChannels,
  //       },
  //     });
  //     nodeRef.current = node;
  //   };

  //   initEngine()
  //     .then(() => {
  //       console.info('Audio engine initialized');
  //       setIsInitialized(true);
  //     })
  //     .catch((error) => {
  //       console.error('Failed to initialize audio engine:', error);
  //       setIsInitialized(false);
  //       throw error;
  //     });

  //   return () => {
  //     if (node) {
  //       node.disconnect();
  //     }
  //     if (ctx) {
  //       ctx.close();
  //     }
  //     engineRef.current = null;
  //     nodeRef.current = null;
  //     audioCtxRef.current = null;
  //   };
  // }, [blockSize, outChannels, sampleRate]);

  useEffect(() => {
    return () => {
      // Cleanup on unmount
      if (nodeRef.current) nodeRef.current.disconnect();
      if (audioCtxRef.current) audioCtxRef.current.close();
      engineRef.current = null;
      nodeRef.current = null;
      audioCtxRef.current = null;
    };
  }, []);

  return {
    engineReady: checkEngineReady(),
    engine: engineRef.current,
    startEngine,
    stopEngine,

    trackState: {
      trackIds,
      addTrack,
      removeTrack,
    },
    play,
    pause,
    seek,
    isPlaying,
    getPlayhead,
  };
};

export default useAudioEngineHook;
