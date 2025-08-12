// initAudioWorklet.ts
import init, { AudioTrans, AudioBuffers } from 'wasm-api';
// inside your component or hook
export const initAndPlayWorklet = async (
  audioContext: AudioContext,
  nbChan: number,
  chan: Float32Array[],
  rate: number
) => {
  // ensure user gesture started audio context (e.g. on button click)
  await audioContext.resume();

  const wasm = await init(); // init wasm_bindgen
  const len = chan[0].length; // frames
  const buffers = new AudioBuffers(len, nbChan);

  // views on wasm memory (byte offset must be number of floats)
  const inputBuf = new Float32Array(wasm.memory.buffer, buffers.input_ptr(), len * nbChan);
  const outputBuf = new Float32Array(wasm.memory.buffer, buffers.output_ptr(), len * nbChan);

  // fill input interleaved
  for (let i = 0; i < len; i++) {
    inputBuf[i * nbChan + 0] = chan[0][i];
    if (nbChan > 1) inputBuf[i * nbChan + 1] = chan[1][i];
  }

  // process once (fills outputBuf)
  buffers.process(AudioTrans.DoubleSpeed);

  // Setup worklet (fallback message-based)
  await audioContext.audioWorklet.addModule('/worklet/queue-output-processor.js');

  const node = new AudioWorkletNode(audioContext, 'queue-output-processor', {
    processorOptions: {
      memory: wasm.memory.buffer,
      outPtr: buffers.output_ptr(),
      len: buffers.len(),
      channels: buffers.channels(),
    },
  });

  // chunking parameters
  const framesPerBlock = 128; // typical worklet block size
  const channels = buffers.channels();
  const framesTotal = buffers.len();
  //   const _floatsPerBlock = framesPerBlock * channels;
  const totalBlocks = Math.ceil(framesTotal / framesPerBlock);

  // pre-fill queue with some blocks to avoid underrun
  const prefillBlocks = Math.min(8, totalBlocks); // push 8 blocks ahead

  // helper to slice block i to a transferable Float32Array
  const sliceBlock = (blockIndex: number) => {
    const startFrame = blockIndex * framesPerBlock;
    const actualFrames = Math.min(framesPerBlock, framesTotal - startFrame);
    const startFloat = startFrame * channels;
    const lengthFloat = actualFrames * channels;
    // copy the subarray into a new Float32Array so we can transfer its buffer
    const copy = new Float32Array(lengthFloat);
    for (let k = 0; k < lengthFloat; k++) {
      copy[k] = outputBuf[startFloat + k];
    }
    return { buffer: copy, frames: actualFrames, channels };
  };

  // queue first prefillBlocks
  for (let b = 0; b < prefillBlocks; b++) {
    if (b >= totalBlocks) break;
    const { buffer, frames } = sliceBlock(b);
    node.port.postMessage({ type: 'push', buffer, frames, channels }, [buffer.buffer]);
  }

  // schedule remaining blocks to be posted a bit ahead of time
  // compute block duration in ms (framesPerBlock / rate * 1000)
  const blockDurationMs = (framesPerBlock / rate) * 1000;
  let nextBlockIndex = prefillBlocks;

  node.connect(audioContext.destination);

  const poster = setInterval(
    () => {
      if (nextBlockIndex >= totalBlocks) {
        clearInterval(poster);
        return;
      }
      const { buffer, frames } = sliceBlock(nextBlockIndex);
      node.port.postMessage({ type: 'push', buffer, frames, channels }, [buffer.buffer]);
      nextBlockIndex++;
    },
    Math.max(5, blockDurationMs * 0.8)
  ); // push slightly faster than real-time

  console.log('Streaming started: totalBlocks=', totalBlocks, 'blockDurationMs=', blockDurationMs);
};
