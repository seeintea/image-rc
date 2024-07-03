use image_rc::{
    compress::compress::compress, load::ImageItem, resize::resize::resize_image as resize,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn resize_image(
    raw: Vec<u8>,
    origin_width: u32,
    origin_height: u32,
    dist_width: u32,
    dist_height: u32,
) -> Vec<u8> {
    let image_item = ImageItem::new(origin_width, origin_height, raw);
    let image = resize(
        image_item.get_image(),
        origin_width,
        origin_height,
        dist_width,
        dist_height,
    );
    image.to_rgb8().to_vec()
}

#[wasm_bindgen]
pub fn resize_image_base64(chars: &str, dist_width: u32, dist_height: u32) -> Vec<u8> {
    let image_item = ImageItem::load_from_base64(chars);
    let origin_image = image_item.get_image();
    let origin_width = origin_image.width();
    let origin_height = origin_image.height();
    let image = resize(
        origin_image,
        origin_width,
        origin_height,
        dist_width,
        dist_height,
    );
    image.to_rgb8().to_vec()
}

#[wasm_bindgen]
pub fn compress_image(raw: Vec<u8>, width: u32, height: u32, quality: u8) -> String {
    let image_item = ImageItem::new(width, height, raw);
    let base64 = compress(image_item, quality);
    base64
}
