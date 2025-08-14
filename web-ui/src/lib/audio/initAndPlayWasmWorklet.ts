import init, { AudioBuffers, AudioTrans } from 'wasm-api';

export async function initAndPlayWasmWorklet(
  audioContext: AudioContext,
  chanCount: number,
  chan: Float32Array[],
  rate: number
) {
  await audioContext.audioWorklet.addModule('/worklet/wasm-output-processor.js');

  const wasm = await init();

  const chanSize = chan[0].length;
  const bufferSize = chanSize * chanCount; // interleaved size

  // Instantiate AudioBuffers with the length and number of channels
  // This will allocate memory in the WASM module
  // and return an instance that we can use to interact with the audio data
  const buffers = new AudioBuffers(chanSize, chanCount, rate);

  // This view allow us to write to the input buffer in WASM memory
  // The input buffer is interleaved, meaning that each channel's samples are stored one
  const inputBuf = new Float32Array(wasm.memory.buffer, buffers.input_ptr(), bufferSize);

  // Fill the input buffer with the audio data as interleaved samples
  for (let i = 0; i < chanSize; i++) {
    inputBuf[i * chanCount + 0] = chan[0][i];
    if (chanCount > 1) inputBuf[i * chanCount + 1] = chan[1][i];
  }

  // Call the WASM processing function with desired transformation
  buffers.process(AudioTrans.NoTransfo);

  buffers.init_ftt(1000, 128); // Initialize FFT with rate and size

  buffers.compute_fft(); // Compute FFT on the input data

  const node = new AudioWorkletNode(audioContext, 'wasm-output-processor', {
    processorOptions: {
      memory: wasm.memory.buffer,
      outPtr: buffers.output_ptr(),
      chanCount,
      chanSize,
      rate,
    },
  });

  node.connect(audioContext.destination);

  console.info('WASM worklet initialized and connected to audio context');

  return { node, buffers, wasm };
}
