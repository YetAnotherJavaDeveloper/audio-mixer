extern crate wasm_bindgen;
use serde_wasm_bindgen::to_value;

use wasm_bindgen::prelude::*;

use audio_core::file::read_music_samples_from_file;

#[wasm_bindgen]
pub fn load_file(path: String) -> Result<JsValue, JsValue> {
    let music_samples = read_music_samples_from_file(path)
        .map_err(|e| JsValue::from_str(&format!("Error: {}", e)))?;

    to_value(&music_samples).map_err(|e| JsValue::from_str(&format!("Serde error: {}", e)))
}
