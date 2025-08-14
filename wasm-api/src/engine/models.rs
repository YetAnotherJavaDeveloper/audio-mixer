#[allow(dead_code)] // Prevent warnings for unused code
use js_sys::SharedArrayBuffer;
use js_sys::{Float32Array, WebAssembly};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum OutputMode {
    Streaming = 0,
    FullBuffer = 1,
}

#[wasm_bindgen]
pub struct TrackHandle {
    id: usize,
    input_ptr: *mut f32,
    frames: usize,
    channels: usize,
}

#[wasm_bindgen]
pub struct RingBuffer {
    buf: Float32Array, // vue JS sur le SAB
    capacity: usize,
    write_idx: usize,
    read_idx: usize,
}

#[wasm_bindgen]
impl RingBuffer {
    #[wasm_bindgen(constructor)]
    pub fn new(capacity: usize) -> RingBuffer {
        // Crée le SAB côté Rust
        // let sab = SharedArrayBuffer::new((capacity * std::mem::size_of::<f32>()) as u32);
        // sab.
        // let buf = Float32Array::new(&sab);

        let buf = Float32Array::new_with_length(capacity as u32);

        RingBuffer {
            buf,
            capacity,
            write_idx: 0,
            read_idx: 0,
        }
    }

    #[wasm_bindgen]
    pub fn push(&mut self, val: f32) {
        self.buf.set_index(self.write_idx as u32, val);
        self.write_idx = (self.write_idx + 1) % self.capacity;
    }

    #[wasm_bindgen]
    pub fn read(&mut self) -> f32 {
        let val = self.buf.get_index(self.read_idx as u32);
        self.read_idx = (self.read_idx + 1) % self.capacity;
        val
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        for i in 0..self.capacity {
            self.buf.set_index(i as u32, 0.0);
        }
        self.write_idx = 0;
        self.read_idx = 0;
    }

    #[wasm_bindgen(getter)]
    pub fn sab(&self) -> SharedArrayBuffer {
        self.buf.buffer().unchecked_into::<SharedArrayBuffer>()
    }

    #[wasm_bindgen(getter)]
    pub fn len(&self) -> usize {
        self.capacity
    }
}

#[derive(Clone, Copy)]
pub struct TrackParams {
    gain: f32,    // Gain in dB
    pan: f32,     // Pan position (-1.0 for left, 0.0 for center, 1.0 for right)
    mute: bool,   // Mute status
    solo: bool,   // Solo status (only this track plays)
    active: bool, // Whether the track is active
}

pub struct Track {
    id: usize,
    ptr: *mut f32,   // Pointer to the track data
    frames: usize,   // Number of frames in the track
    channels: usize, // Number of channels in the track
    params: TrackParams,
    cursor: usize, // Current position in samples
}

impl TrackHandle {
    pub fn new(id: usize, input_ptr: *mut f32, frames: usize, channels: usize) -> TrackHandle {
        TrackHandle {
            id,
            input_ptr,
            frames,
            channels,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn input_ptr(&self) -> *mut f32 {
        self.input_ptr
    }

    pub fn frames(&self) -> usize {
        self.frames
    }

    pub fn channels(&self) -> usize {
        self.channels
    }
}

impl TrackParams {
    pub fn new(gain: f32, pan: f32, mute: bool, solo: bool) -> TrackParams {
        TrackParams {
            gain,
            pan,
            mute,
            solo,
            active: true, // Default to active
        }
    }

    pub fn gain(&self) -> f32 {
        self.gain
    }
    pub fn pan(&self) -> f32 {
        self.pan
    }
    pub fn mute(&self) -> bool {
        self.mute
    }
    pub fn solo(&self) -> bool {
        self.solo
    }
    pub fn active(&self) -> bool {
        self.active
    }

    pub fn set_gain(&mut self, gain: f32) {
        self.gain = gain;
    }
    pub fn set_pan(&mut self, pan: f32) {
        self.pan = pan;
    }
    pub fn set_mute(&mut self, mute: bool) {
        self.mute = mute;
    }
    pub fn set_solo(&mut self, solo: bool) {
        self.solo = solo;
    }
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

impl Track {
    pub fn new(
        id: usize,
        ptr: *mut f32,
        frames: usize,
        channels: usize,
        params: TrackParams,
    ) -> Track {
        Track {
            id,
            ptr,
            frames,
            channels,
            params,
            cursor: 0,
        }
    }

    // -- Getters for track parameters
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn ptr(&self) -> *mut f32 {
        self.ptr
    }

    pub fn frames(&self) -> usize {
        self.frames
    }

    pub fn channels(&self) -> usize {
        self.channels
    }

    pub fn params(&self) -> &TrackParams {
        &self.params
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn set_cursor(&mut self, cursor: usize) {
        self.cursor = cursor;
    }

    pub fn params_mut(&mut self) -> &mut TrackParams {
        &mut self.params
    }

    pub unsafe fn as_slice(&self) -> &[f32] {
        let len = self.frames * self.channels;
        unsafe { std::slice::from_raw_parts(self.ptr, len) }
    }
}
