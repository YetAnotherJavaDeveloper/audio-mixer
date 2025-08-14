use wasm_bindgen::prelude::*;

use audio_core::core::{
    FftDefinition, MultiChannelSample, Rate, Sample, fft_to_frequencies, test,
    transform_double_speed,
};

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct TrackConfig {
    pub sample_rate: u32,
    pub channels: usize,
    pub gain: f32,
    pub speed: f32,
    pub input_ptr: *mut f32,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct AudioEngineState {
    pub is_initialized: *mut bool, // Whether the audio engine is initialized
    pub is_playing: *mut bool,     // Whether the audio engine is currently playing
    pub with_transfo: *mut bool, // Whether to apply configured transformations or publish raw audio
    pub track_config: TrackConfig, // Configuration for the audio track
    pub fft_initialized: *mut bool, // Whether the FFT is initialized
}

#[wasm_bindgen]
pub struct AudioBuffers {
    pub input_ptr: *mut f32,
    pub output_ptr: *mut f32,
    pub fft_ptr: *mut f32, // FFT output pointer for output mixed buffer
    pub fft_size: usize,
    pub fft_rate: u32, // Sample rate for FFT
    pub len: usize,
    pub current_frame: usize, // Current frame index for processing
    pub channels: usize,
    pub engine_state: AudioEngineState,
}

// Like Transformation enum from audio_core::core::models.rs
#[wasm_bindgen]
pub enum AudioTrans {
    DoubleSpeed,
    NoTransfo,
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn convert_channels_to_output(channels: &Vec<Vec<f32>>) -> Vec<f32> {
    let ouput_size = channels[0].len() * channels.len();
    let channel_size = channels[0].len();
    let channel_count = channels.len();
    let mut output_slice = Vec::with_capacity(ouput_size);

    for i in 0..channel_size {
        for c in 0..channel_count {
            output_slice[i * channel_count + c] = channels[c][i];
        }
    }

    output_slice
}

#[wasm_bindgen]
impl AudioBuffers {
    #[wasm_bindgen(constructor)]
    pub fn new(len: usize, channels: usize, rate: u32) -> AudioBuffers {
        let size = len * channels;
        let fft_default_size = 128 * 2 + 1; // Default FFT size, can be adjusted (128 * 2 + 1 for frequency and magnitude pairs)
        // Allocate input and output buffers on heap
        let mut input = Vec::with_capacity(size);
        let mut output = Vec::with_capacity(size);
        let mut fft_output = Vec::with_capacity(fft_default_size);
        // Fill with zeros for safety
        input.resize(size, 0.0);
        output.resize(size, 0.0);

        // Leak vec to get stable pointer for WASM memory sharing
        let input_ptr = input.as_mut_ptr();
        let output_ptr = output.as_mut_ptr();
        let fft_ptr = fft_output.as_mut_ptr();
        // Memory ownership will be transferred to JS, this prevents Rust from freeing it
        std::mem::forget(input);
        std::mem::forget(output);
        std::mem::forget(fft_output);

        let engine_state = AudioEngineState {
            is_initialized: Box::into_raw(Box::new(false)),
            is_playing: Box::into_raw(Box::new(false)),
            with_transfo: Box::into_raw(Box::new(false)),
            fft_initialized: Box::into_raw(Box::new(false)),
            track_config: TrackConfig {
                sample_rate: rate,
                channels,
                gain: 1.0,  // Default gain
                speed: 1.0, // Default speed
                input_ptr,
            },
        };

        AudioBuffers {
            input_ptr,
            output_ptr,
            len,
            channels,
            fft_ptr,
            fft_size: fft_default_size,
            fft_rate: 100,
            current_frame: 0,
            engine_state,
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

    pub fn set_current_frame(&mut self, frame: usize) {
        self.current_frame = frame;
    }

    // Dispatch methods for transformations
    pub fn process(&self, transformation: AudioTrans) {
        match transformation {
            AudioTrans::DoubleSpeed => self.wrap_double_speed(),
            AudioTrans::NoTransfo => unsafe {
                let size = self.len * self.channels;
                let input_slice = std::slice::from_raw_parts(self.input_ptr, size);
                let output_slice = std::slice::from_raw_parts_mut(self.output_ptr, size);

                for i in 0..size {
                    output_slice[i] = input_slice[i];
                }
            },
        }
    }

    pub fn wrap_double_speed(&self) {
        unsafe {
            let size = self.len * self.channels;
            let input_slice = std::slice::from_raw_parts(self.input_ptr, size);
            let mut channels: Vec<Vec<f32>> = vec![Vec::with_capacity(self.len); self.channels];

            for i in 0..self.len {
                for c in 0..self.channels {
                    channels[c].push(input_slice[i * self.channels + c]);
                }
            }

            let multi_channel_sample = MultiChannelSample::new(vec![
                Sample::new(channels[0].clone()),
                Sample::new(channels[1].clone()),
            ]);

            let result: MultiChannelSample = transform_double_speed(&multi_channel_sample);

            let output_interleaved = result.to_interleaved();
            let output_size = output_interleaved.len();
            let output_slice = std::slice::from_raw_parts_mut(self.output_ptr, output_size);
            for i in 0..output_size {
                output_slice[i] = output_interleaved[i];
            }
        }
    }

    pub fn init_ftt(&mut self, fft_size: usize, sample_rate: u32) {
        unsafe {
            // free previous fft_ptr if it exists
            if !self.fft_ptr.is_null() {
                let _ = Vec::from_raw_parts(self.fft_ptr, self.fft_size, self.fft_size);
            }

            self.fft_size = fft_size * 2 + 1; // Frequency and magnitude pairs
            self.fft_rate = sample_rate;

            let mut fft_output = Vec::with_capacity(self.fft_size);
            fft_output.resize(self.fft_size, 0.0); // Initialize with zeros
            // Leak vec to get stable pointer for WASM memory sharing
            self.fft_ptr = fft_output.as_mut_ptr();
            std::mem::forget(fft_output);

            self.engine_state.fft_initialized = Box::into_raw(Box::new(true));
        }
    }

    pub fn compute_fft(&self) {
        unsafe {
            log("Computing FFT...");
            if self.engine_state.fft_initialized.is_null() || !*self.engine_state.fft_initialized {
                log("FFT not initialized. Call init_fft first.");
                return;
            }

            let fft_size = self.fft_size;
            let chunk_size = 128 * self.channels; // Example chunk size, can be adjusted
            let start_index = self.current_frame;
            let end_index = start_index + chunk_size;

            if end_index > self.len {
                log(&format!(
                    "Not enough samples to compute FFT. Current frame: {}, chunk size: {}, total length: {}",
                    start_index, chunk_size, self.len
                ));
                return;
            }
            let input_slice = std::slice::from_raw_parts(self.output_ptr, self.len * self.channels);

            let input_chunk = &input_slice[start_index..end_index];
            let fft_definition = FftDefinition::for_frequency_precision(self.fft_rate);
            let sample = Sample::new(input_chunk.to_vec());

            log(&format!(
                "Computing FFT for {} samples at rate {} Hz",
                sample.len(),
                self.fft_rate
            ));
            log(test().as_str());
            let rate = Rate::new(self.engine_state.track_config.sample_rate);

            log(&format!(
                "FFT Definition: Start Frequency: {}, End Frequency: {}, Precision: {}, Sample Rate: {}",
                fft_definition.start_frequency(),
                fft_definition.end_frequency(),
                fft_definition.frequency_precision(),
                rate.value()
            ));

            let fft_result = fft_to_frequencies(&sample, &rate, &fft_definition).unwrap_throw();

            log(&format!("FFT computed with {} results", fft_result.len()));
            // Fill fft_ptr with (freq, magnitude) pairs
            let fft_output_slice = std::slice::from_raw_parts_mut(self.fft_ptr, fft_size);
            for (i, result) in fft_result.iter().enumerate() {
                if i >= fft_size {
                    break; // Avoid overflow if there are more results than fft_size
                }
                fft_output_slice[i * 2] = result.frequency();
                fft_output_slice[i * 2 + 1] = result.magnitude();
            }
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

    pub fn free_fft(&mut self) {
        // Free the memory allocated for FFT output
        unsafe {
            if !self.fft_ptr.is_null() {
                let _ = Vec::from_raw_parts(self.fft_ptr, self.fft_size, self.fft_size);
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
