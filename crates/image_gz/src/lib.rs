extern crate console_error_panic_hook;
extern crate wasm_bindgen;

use image_tool;
use wasm_bindgen::prelude::*;

#[cfg(debug_assertions)]
use console_error_panic_hook::set_once;

#[wasm_bindgen]
pub fn resize(
    raw: Vec<u8>,
    origin_width: u32,
    origin_height: u32,
    dist_width: u32,
    dist_height: u32,
) -> Vec<u8> {
    #[cfg(debug_assertions)]
    set_once();
    let image = image_tool::resize(raw, origin_width, origin_height, dist_width, dist_height);
    image.to_rgb8().to_vec()
}
