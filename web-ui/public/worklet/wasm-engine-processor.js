class WasmEngineProcessor extends AudioWorkletProcessor {
    constructor(options) {
        super();

        const { memory, outPtr, blockSize, outChannels } = options.processorOptions;

        this.blockSize = blockSize;
        this.outChannels = outChannels;
        this.outPtr = outPtr;
        this.memory = memory;
        this.blockIdx = 0; // Index to track the current block in the ring buffer
        this.bufferIdx = 0; // Index to track the current position in the output buffer

        const blockBufferSize = this.blockSize * this.outChannels; // 128 samples * 2 channels
        const ringBufferSize = blockBufferSize * 4; // Assuming 4 blocks in the ring buffer

        // this.outputView = new Float32Array(this.memory, this.outPtr, bufferSize);

        // Create a view for the buffer block to read from

        this.outputView = new Float32Array(this.memory, this.outPtr, ringBufferSize);
        console.table({
            ringBufferSize,
            outPtr,
            blockSize,
            outChannels,
            memory: this.outputView.length

        })
        // Handle messages from main thread (play/pause/seek)
        this.port.onmessage = (event) => {
            const data = event.data;
            console.log('WasmEngineProcessor received message:', data);
            if (data.play !== undefined) this.playing = data.play;
            if (data.seek !== undefined) this.playIndex = data.seek % this.blockSize;
        };
    }

    process(_inputs, outputs) {
        const outL = outputs[0][0];
        const outR = outputs[0][1] || outputs[0][0];

        // dans process()
        let idx = this.bufferIdx;
        for (let i = 0; i < this.blockSize; i++) {
            outL[i] = this.outputView[idx];
            outR[i] = this.outChannels > 1 ? this.outputView[idx + 1] : outL[i];

            idx += this.outChannels;
            if (idx >= this.outputView.length) idx = 0;

            if (i % this.blockSize === 0) {
                console.log(`Buffer : ${this.outputView}`)
            }
        }
        this.bufferIdx = idx; // mise à jour pour le prochain process()
        this.port.postMessage({ request: 'nextBlock' });
        return true;
    }

}

registerProcessor('wasm-engine-processor', WasmEngineProcessor);