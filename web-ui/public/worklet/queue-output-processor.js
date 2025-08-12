class QueueOutputProcessor extends AudioWorkletProcessor {
    constructor() {
        super();
        this.queue = [];

        this.port.onmessage = (event) => {
            if (event.data.type === 'push') {
                this.queue.push(event.data.buffer);
            }
        };
    }

    process(_inputs, outputs) {
        const outL = outputs[0][0];
        const outR = outputs[0][1] || outputs[0][0];

        if (this.queue.length === 0) {
            outL.fill(0);
            outR.fill(0);
            return true;
        }

        const block = this.queue.shift(); // Float32Array interleaved
        for (let i = 0; i < outL.length; i++) {
            outL[i] = block[i * 2] || 0;
            outR[i] = block[i * 2 + 1] || 0;
        }
        return true;
    }
}

registerProcessor('queue-output-processor', QueueOutputProcessor);
