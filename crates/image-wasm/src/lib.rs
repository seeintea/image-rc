use wasm_bindgen::prelude::*;

use image_rc::{load_form_buffer, load_from_base64, rc_compress, rc_resize};

#[wasm_bindgen(js_name = "resizeImageData")]
pub fn resize_common(data: Vec<u8>, dist_width: u32, dist_height: u32) -> Vec<u8> {
    let converter = load_form_buffer(data);
    let converter = rc_resize(&converter, dist_width, dist_height);
    converter.buffer()
}

#[wasm_bindgen(js_name = "resizeDataURL")]
pub fn resize_base64(data: &str, dist_width: u32, dist_height: u32) -> Vec<u8> {
    let converter = load_from_base64(data);
    let converter = rc_resize(&converter, dist_width, dist_height);
    converter.buffer()
}

#[wasm_bindgen(js_name = "compressImageData")]
pub fn compress_common(data: Vec<u8>, quality: u8) -> Vec<u8> {
    let converter = load_form_buffer(data);
    let converter = rc_compress(&converter, quality);
    converter.buffer()
}

#[wasm_bindgen(js_name = "compressDataURL")]
pub fn compress_base64(data: &str, quality: u8) -> Vec<u8> {
    let converter = load_from_base64(data);
    let converter = rc_compress(&converter, quality);
    converter.buffer()
}
