pub mod audio;

// extern crate wasm_bindgen;
use js_sys::{ArrayBuffer, Float32Array, JsString};
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use audio_core::core::{MultiChannelSample, MusicSample, Sample, Rate};

#[derive(Serialize)]
pub struct Response {
    pub code: u16,
    pub message: String,
}

// This defines the Node.js Buffer type
#[wasm_bindgen]
extern "C" {
    pub type Buffer;

    #[wasm_bindgen(method, getter)]
    fn buffer(this: &Buffer) -> ArrayBuffer;

    #[wasm_bindgen(method, getter, js_name = byteOffset)]
    fn byte_offset(this: &Buffer) -> f32;

    #[wasm_bindgen(method, getter)]
    fn length(this: &Buffer) -> f32;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[wasm_bindgen]
#[derive(Tsify)]
pub struct MusicSampleDto {
    #[tsify(type = "Float32Array[]")]
    all_samples: js_sys::Array, // Array of Float32Array
    sample_rate: u32,
}

#[wasm_bindgen]
impl MusicSampleDto {
    #[wasm_bindgen(getter)]
    pub fn all_samples(&self) -> js_sys::Array {
        return self.all_samples.clone();
    }

    #[wasm_bindgen(getter)]
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    #[wasm_bindgen(constructor)]
    pub fn new(all_samples: js_sys::Array, sample_rate: u32) -> MusicSampleDto {
        MusicSampleDto {
            all_samples,
            sample_rate,
        }
    }
}

#[wasm_bindgen]
pub fn hello() -> JsString {
    return JsString::from("Hello from Rust!");
}

#[wasm_bindgen]
pub fn handle_music_sample(dto: MusicSampleDto) -> JsValue {
    let chan_0 = dto.all_samples.get(0).unchecked_into::<Float32Array>();
    let chan_1 = dto.all_samples.get(1).unchecked_into::<Float32Array>();
    let rate = dto.sample_rate();

    // assert not empty
    if chan_0.length() == 0 || chan_1.length() == 0 {
        return JsValue::from_str("Error: Channels are empty");
    }

    // Create a MusicSample from the Float32Array
    let sample_chan_0 = Sample::new(chan_0.to_vec());
    let sample_chan_1 = Sample::new(chan_1.to_vec());
    let sample_rate = Rate::new(rate);
    let multi_channel_sample = MultiChannelSample::new(vec![sample_chan_0, sample_chan_1]);
    let music_sample = MusicSample::new(multi_channel_sample, sample_rate);

    log(&format!(
        "Received music sample with {:?} channels at {} Hz",
        music_sample.multi_channel_sample().channels(),
        music_sample.sample_rate().value()
    ));

    let response = Response {
        code: 200,
        message: format!(
            "Music sample processed successfully - {} samples at {} Hz",
            music_sample.multi_channel_sample().first_channel().len(),
            music_sample.sample_rate().value()
        ),
    };

    let r = to_value(&response).map_err(|e| JsError::new(&format!("Serde error: {}", e)));

    return r.unwrap_or_else(|_| JsValue::from_str(&format!("Error during serialization")));
}

#[wasm_bindgen]
pub fn computation(buffer: Float32Array) -> JsValue {
    let out: Vec<f32> = buffer.to_vec();
    let len = out.len();
    let buffLen = buffer.length() as usize;

    let response = Response {
        code: 200,
        message: format!("Success - Computation completed - Length of data: {}", len),
    };

    log(&format!(
        "Buffer length: {}, Float32Array length: {}",
        buffLen, len
    ));
    log(&response.message);

    let r = to_value(&response).map_err(|e| JsError::new(&format!("Serde error: {}", e)));

    return r.unwrap_or_else(|_| JsValue::from_str(&format!("Error during serialization")));
}
