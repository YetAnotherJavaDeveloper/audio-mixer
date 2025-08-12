use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct AudioBuffers {
    pub input_ptr: *mut f32,
    pub output_ptr: *mut f32,
    pub len: usize,
    pub channels: usize,
}

// Like Transformation enum from audio_core::core::models.rs
#[wasm_bindgen]
pub enum AudioTrans {
    DoubleSpeed,
}

#[wasm_bindgen]
impl AudioBuffers {
    #[wasm_bindgen(constructor)]
    pub fn new(len: usize, channels: usize) -> AudioBuffers {
        let size = len * channels;
        // Allocate input and output buffers on heap
        let mut input = Vec::with_capacity(size);
        let mut output = Vec::with_capacity(size);
        // Fill with zeros for safety
        input.resize(size, 0.0);
        output.resize(size, 0.0);

        // Leak vec to get stable pointer for WASM memory sharing
        let input_ptr = input.as_mut_ptr();
        let output_ptr = output.as_mut_ptr();
        // Memory ownership will be transferred to JS, this prevents Rust from freeing it
        std::mem::forget(input);
        std::mem::forget(output);

        AudioBuffers {
            input_ptr,
            output_ptr,
            len,
            channels,
        }
    }

    pub fn input_ptr(&self) -> *mut f32 {
        self.input_ptr
    }

    pub fn output_ptr(&self) -> *mut f32 {
        self.output_ptr
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn channels(&self) -> usize {
        self.channels
    }

    // Dispatch methods for transformations
    pub fn process(&self, transformation: AudioTrans) {
        match transformation {
            AudioTrans::DoubleSpeed => self.apply_double_speed(),
        }
    }

    pub fn apply_gain(&self, gain: f32) {
        unsafe {
            let size = self.len * self.channels;
            let input_slice = std::slice::from_raw_parts(self.input_ptr, size);
            let output_slice = std::slice::from_raw_parts_mut(self.output_ptr, size);

            for i in 0..size {
                output_slice[i] = input_slice[i] * gain;
            }
        }
    }

    pub fn apply_double_speed(&self) {
        unsafe {
            let size = self.len * self.channels;
            let input_slice = std::slice::from_raw_parts(self.input_ptr, size);
            let output_slice = std::slice::from_raw_parts_mut(self.output_ptr, size / 2);

            for i in 0..(size / 2) {
                output_slice[i] = input_slice[i * 2];
            }
        }
    }

    pub fn free_all(&mut self) {
        // Free the memory allocated for input and output buffers
        unsafe {
            if !self.input_ptr.is_null() {
                let _ = Vec::from_raw_parts(
                    self.input_ptr,
                    self.len * self.channels,
                    self.len * self.channels,
                );
            }
            if !self.output_ptr.is_null() {
                let _ = Vec::from_raw_parts(
                    self.output_ptr,
                    self.len * self.channels,
                    self.len * self.channels,
                );
            }
        }
    }
}
