// extern crate wasm_bindgen;
use js_sys::{ArrayBuffer, Float32Array, JSON, JsString};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

// use audio_core::core::MusicSamples;

// use audio_core::file::read_music_samples_from_file;

// #[wasm_bindgen]
// pub fn load_file(path: String) -> Result<JsValue, JsValue> {
//     let music_samples = read_music_samples_from_file(path)
//         .map_err(|e| JsValue::from_str(&format!("Error: {}", e)))?;

//     to_value(&music_samples).map_err(|e| JsValue::from_str(&format!("Serde error: {}", e)))
// }

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
    fn byte_offset(this: &Buffer) -> u32;

    #[wasm_bindgen(method, getter)]
    fn length(this: &Buffer) -> u32;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn hello() -> JsString {
    return JsString::from("Hello from Rust!");
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

    let r = to_value(&response).map_err(|e| JsError::new(&format!("Serde error: {}", e)));

    log(&format!(
        "Buffer length: {}, Float32Array length: {}",
        buffLen, len
    ));
    log(&response.message);

    return r.unwrap_or_else(|_| JsValue::from_str(&format!("Error during serialization")));
}
