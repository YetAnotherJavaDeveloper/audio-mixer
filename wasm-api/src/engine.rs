mod models;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

use std::mem;

use js_sys::{Float32Array, SharedArrayBuffer};
use wasm_bindgen::prelude::*;

use models::{OutputMode, RingBuffer, Track, TrackHandle};

use crate::engine::models::TrackParams;

const DEFAULT_RING_BUFFER_BLOCKS: usize = 4; // Number of blocks in the ring buffer

#[wasm_bindgen]
pub struct Engine {
    sample_rate: u32,    // Sample rate in Hz
    block_size: usize,   // frames per block (default 128)
    out_channels: usize, // Number of output channels (default 2)

    // Tracks
    tracks: Vec<Track>,

    // Intern mix buffer
    mix_buf: Vec<f32>, // size = block_size * out_channels

    // Transport
    playing: bool,
    playhead: usize, // frame globale (avance par blocks)

    output_ptr: *const f32,
    output_mode: OutputMode,
    ring_buffer: RingBuffer,
    full_output: Vec<f32>,
}

#[wasm_bindgen]
impl Engine {
    /// Create a new Engine instance
    #[wasm_bindgen(constructor)]
    pub fn new(
        sample_rate: u32,
        out_channels: usize,
        block_size: usize,
        ring_buffer: RingBuffer,
    ) -> Engine {
        let size = block_size * out_channels;
        // let (output_ptr, ring_buffer) = Engine::init_ring_buffer(block_size, out_channels);

        Engine {
            sample_rate,
            block_size,
            out_channels,
            tracks: Vec::new(),
            mix_buf: vec![0.0; size],
            playing: false,
            playhead: 0,
            output_ptr: std::ptr::null(), // Will be set later
            output_mode: OutputMode::Streaming,
            ring_buffer: ring_buffer,
            full_output: Vec::new(),
        }
    }

    // fn init_ring_buffer(block_size: usize, channels: usize) -> (*const f32, RingBuffer) {
    //     let mut ring_buffer = RingBuffer::new((block_size * channels) * DEFAULT_RING_BUFFER_BLOCKS);
    //     let ptr = ring_buffer.ptr();

    //     // return the ring buffer with its pointer
    //     return (ptr, ring_buffer);
    // }

    // -- Transport control --
    pub fn play(&mut self) {
        self.playing = true;
    }

    pub fn pause(&mut self) {
        self.playing = false;
    }

    pub fn is_playing(&self) -> bool {
        self.playing
    }

    pub fn seek(&mut self, frame: usize) {
        self.playhead = frame;
        for tr in &mut self.tracks {
            let cursor = frame.min(tr.frames());
            tr.set_cursor(cursor);
        }
    }

    // -- Getters for JS side --
    #[wasm_bindgen(getter)]
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    #[wasm_bindgen(getter)]
    pub fn output_ptr(&self) -> *const f32 {
        self.output_ptr
    }
    #[wasm_bindgen(getter)]
    pub fn block_size(&self) -> usize {
        self.block_size
    }
    #[wasm_bindgen(getter)]
    pub fn out_channels(&self) -> usize {
        self.out_channels
    }
    #[wasm_bindgen(getter)]
    pub fn playhead(&self) -> usize {
        self.playhead
    }
    #[wasm_bindgen(getter)]
    pub fn ring_buffer_len(&self) -> usize {
        self.ring_buffer.len()
    }
    #[wasm_bindgen(getter)]
    pub fn ring_buffer_sab(&self) -> SharedArrayBuffer {
        self.ring_buffer.sab()
    }

    // #[wasm_bindgen(getter)]
    // pub fn ring_buffer_read_idx(&self) -> usize {
    //     self.ring_buffer.read_idx
    // }
    // pub fn read_from_ring_buffer(&mut self, js_array: &Float32Array, count: usize) {
    //     let mut temp_buf = vec![0.0; count * self.out_channels];
    //     for i in 0..count {
    //         temp_buf[i] = self.ring_buffer.read();
    //     }
    //     log(&format!(
    //         "Read {} samples from ring buffer, first sample: {}",
    //         count, temp_buf[0]
    //     ));
    //     js_array.copy_from(&temp_buf);
    // }

    pub fn get_full_ouput(&mut self) -> Float32Array {
        let arr = Float32Array::new_with_length(self.full_output.len() as u32);
        arr.copy_from(&self.full_output);
        arr
    }

    // -- Track management --
    #[wasm_bindgen]
    pub fn add_track(&mut self, frames: usize, channels: usize) -> usize {
        let total_len = frames * channels;
        let mut buf = Vec::<f32>::with_capacity(total_len);
        let ptr = buf.as_mut_ptr();
        std::mem::forget(buf); // Prevent Rust from freeing the buffer

        let id = self.tracks.len(); // Simple ID based on index
        let track = Track::new(
            id,
            ptr,
            frames,
            channels,
            TrackParams::new(1.0, 0.5, false, false),
        );

        self.tracks.push(track);

        TrackHandle::new(id, ptr, frames, channels);
        id
    }
    #[wasm_bindgen]
    pub fn remove_track(&mut self, track_id: usize) {
        if let Some(idx) = self.tracks.iter().position(|t| t.id() == track_id) {
            let t = self.tracks.remove(idx);
            unsafe {
                let _ = Vec::from_raw_parts(
                    t.ptr(),
                    t.frames() * t.channels(),
                    t.frames() * t.channels(),
                );
            }
        }
    }
    #[wasm_bindgen]
    pub fn write_to_track(
        &mut self,
        track_id: usize,
        buffer: &Float32Array,
        offset: usize,
    ) -> Result<(), JsValue> {
        if let Some(track) = self.tracks.get_mut(track_id) {
            let total_samples = track.frames() * track.channels();

            if buffer.length() as usize + offset > total_samples {
                return Err(JsValue::from_str("Buffer overflow"));
            }

            let src: Vec<f32> = buffer.to_vec();
            let dst = unsafe { std::slice::from_raw_parts_mut(track.ptr(), total_samples) };

            dst[offset..offset + src.len()].copy_from_slice(&src);

            Ok(())
        } else {
            Err(JsValue::from_str("Track not found"))
        }
    }

    pub fn set_track_active(&mut self, track_id: usize, active: bool) {
        if let Some(t) = self.tracks.get_mut(track_id) {
            t.params_mut().set_active(active);
        }
    }
    pub fn set_track_gain(&mut self, track_id: usize, gain: f32) {
        if let Some(t) = self.tracks.get_mut(track_id) {
            t.params_mut().set_gain(gain);
        }
    }
    pub fn set_track_mute(&mut self, track_id: usize, mute: bool) {
        if let Some(t) = self.tracks.get_mut(track_id) {
            t.params_mut().set_mute(mute);
        }
    }
    pub fn set_track_solo(&mut self, track_id: usize, solo: bool) {
        if let Some(t) = self.tracks.get_mut(track_id) {
            t.params_mut().set_solo(solo); // Maybe should update all other tracks' solo state to false
        }
    }

    pub fn set_track_pan(&mut self, track_id: usize, pan: f32) {
        if let Some(t) = self.tracks.get_mut(track_id) {
            t.params_mut().set_pan(pan.clamp(-1.0, 1.0));
        }
    }

    // -- Output control --
    pub fn set_output_mode(&mut self, mode: OutputMode) {
        self.clear_output();

        self.output_mode = mode;
    }

    pub fn process_block(&mut self) {
        if !self.playing {
            return;
        }

        // 1. Clear the mix buffer
        self.mix_buf.fill(0.0);

        // 2. Basic mixing multi tracks with balance and gain
        for track_idx in self.get_active_track_indices() {
            let track = &self.tracks[track_idx];
            if track.params().mute() {
                continue; // Skip muted tracks
            }

            log(&format!("Processing track {}", track.id()));

            let (gain_l, gain_r) = self.get_gain_pan(track);

            let track_buf = unsafe { track.as_slice() };
            let cursor = track.cursor();

            let frames_to_mix = self.block_size.min(track.frames() - cursor);

            log(&format!(
                "Track {}: cursor {}, frames to mix {}",
                track.id(),
                cursor,
                frames_to_mix
            ));

            for frame_idx in 0..frames_to_mix {
                let src_idx = cursor + frame_idx;
                let dest_idx = frame_idx * self.out_channels;

                if self.out_channels == 2 {
                    let sample_l = track_buf[src_idx];
                    let sample_r = if track.channels() > 1 {
                        track_buf[src_idx + 1]
                    } else {
                        sample_l // Mono track, duplicate left channel
                    };

                    self.mix_buf[dest_idx] += sample_l * gain_l;
                    self.mix_buf[dest_idx + 1] += sample_r * gain_r;
                } else {
                    let sample_l = track_buf[src_idx];
                    self.mix_buf[dest_idx] += sample_l * gain_l;
                }
            }

            log(&format!(
                "Track {}: Mixed {} frames into buffer of size {} with first sample {}",
                track.id(),
                frames_to_mix,
                self.mix_buf.len(),
                self.mix_buf[0]
            ));

            // 3. Update the track cursor
            self.tracks[track_idx].set_cursor(cursor + frames_to_mix);
        }

        // 4. Write the mix buffer to the output
        match self.output_mode {
            OutputMode::Streaming => {
                for &s in &self.mix_buf {
                    self.ring_buffer.push(s);
                }
            }
            OutputMode::FullBuffer => {
                self.full_output.extend_from_slice(&self.mix_buf);
            }
        }

        // 5. Update the playhead
        self.playhead += self.block_size;
        // 6. If mode is Streaming, align the output pointer on the block on the ring buffer
    }

    // -- Helpers --
    fn get_active_track_indices(&self) -> Vec<usize> {
        let active: Vec<usize> = self
            .tracks
            .iter()
            .enumerate()
            .filter(|(_, t)| t.params().active())
            .map(|(i, _)| i)
            .collect();

        if active.iter().any(|&i| self.tracks[i].params().solo()) {
            active
                .into_iter()
                .filter(|&i| self.tracks[i].params().solo())
                .collect()
        } else {
            active
        }
    }

    fn get_gain_pan(&self, track: &Track) -> (f32, f32) {
        let gain = track.params().gain();
        let pan = track.params().pan();

        let (gain_l, gain_r) = if self.out_channels == 2 {
            let l = (1.0 - pan).clamp(0.0, 1.0);
            let r = (1.0 + pan).clamp(0.0, 1.0);
            (gain * l, gain * r)
        } else {
            (gain, gain)
        };
        (gain_l, gain_r)
    }

    fn clear_output(&mut self) {
        // Clear all output buffers
        unsafe {
            let _ = Vec::from_raw_parts(
                self.output_ptr as *mut f32,
                self.block_size * self.out_channels * DEFAULT_RING_BUFFER_BLOCKS,
                self.block_size * self.out_channels * DEFAULT_RING_BUFFER_BLOCKS,
            );
            self.output_ptr = std::ptr::null_mut();
        }
        self.full_output.clear();
        // self.ring_buffer.reset();
        self.playhead = 0;
        self.mix_buf.fill(0.0);
    }
}
