extern crate console_error_panic_hook;
extern crate wasm_bindgen;

use image_tool;
use wasm_bindgen::prelude::*;

#[cfg(debug_assertions)]
use console_error_panic_hook::set_once;

#[wasm_bindgen]
pub fn resize_by_image_data(
    raw: Vec<u8>,
    origin_width: u32,
    origin_height: u32,
    dist_width: u32,
    dist_height: u32,
) -> Vec<u8> {
    #[cfg(debug_assertions)]
    set_once();
    image_tool::resize_image_by_image_data(
        raw,
        origin_width,
        origin_height,
        dist_width,
        dist_height,
    )
}
