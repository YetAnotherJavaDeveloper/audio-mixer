class WasmEngineProcessor extends AudioWorkletProcessor {
    constructor(options) {
        super();

        const { ringBuffer, blockSize, outChannels } = options.processorOptions;

        this.blockSize = blockSize;
        this.outChannels = outChannels;
        this.ringBuffer = ringBuffer;
        this.blockIdx = 0; // Index to track the current block in the ring buffer
        this.bufferIdx = 0; // Index to track the current position in the output buffer

        // Create a view for the buffer block to read from
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
            outL[i] = this.ringBuffer[idx];
            outR[i] = this.outChannels > 1 ? this.ringBuffer[idx + 1] : outL[i];

            idx += this.outChannels;
            if (idx >= this.ringBuffer.length) idx = 0;

            if (i % this.blockSize === 0) {
                console.log(`Buffer : ${this.ringBuffer}`)
            }
        }
        if (this.bufferIdx >= this.ringBuffer.length) {
            this.bufferIdx = 0; // Reset buffer index if it exceeds the ring buffer length
        } else {
            this.bufferIdx = idx; // mise à jour pour le prochain process()

        }
        this.port.postMessage({ request: 'nextBlock' });
        return true;
    }

}

registerProcessor('wasm-engine-processor_v2', WasmEngineProcessor);