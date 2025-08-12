class WasmOutputProcessor extends AudioWorkletProcessor {
  constructor(options) {
    super();

    const { memory, outPtr, chanCount, rate, chanSize } = options.processorOptions;

    this.playIndex = 0; // Index to track the current position in the output buffer
    this.blockSize = 128; // Assuming a block size of 128 samples
    this.sampleRate = rate || 44100; // Default sample rate
    this.outPtr = outPtr; // Pointer to the output buffer in WebAssembly memory
    this.chanSize = chanSize; // Size of the channel buffer
    this.chanCount = chanCount; // Number of channels
    this.memory = memory; // WebAssembly memory buffer

    const bufferSize = this.chanSize * this.chanCount; // Interleaved size
    // Note: the size in based on input buffer size, which may differ in reality (e.g. speed up/down transform)
    this.outputView = new Float32Array(this.memory, this.outPtr, bufferSize);

    console.log(`Output view created with length: ${this.outputView.length}`);
  }

  process(_inputs, outputs) {
    const outL = outputs[0][0];
    const outR = outputs[0][1] || outputs[0][0];
    const blockSize = outL.length;

    for (let i = 0; i < this.blockSize; i++) {
      const frameIdx = (this.playIndex + i) % this.chanSize;
      const sampleIdx = frameIdx * this.chanCount;
      outL[i] = this.outputView[sampleIdx] || 0;
      outR[i] = this.outputView[sampleIdx + 1] || 0;
    }

    this.playIndex = (this.playIndex + blockSize) % this.chanSize;

    this.port.postMessage({ currentFrame: this.playIndex });

    this.port.onmessage = (event) => {
      if (event.data.seek !== undefined) {
        console.log(`Seeking to frame: ${event.data.seek}`);
        this.playIndex = event.data.seek % this.chanSize; // Reset play index to the seek position
      }
    };

    return true;
  }
}

registerProcessor('wasm-output-processor', WasmOutputProcessor);
