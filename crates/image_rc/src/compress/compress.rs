use std::io::Cursor;

use base64::{engine::general_purpose, Engine as _};

use image::{load_from_memory, ImageOutputFormat};

use crate::load::ImageItem;

use super::utils::is_rgba8;

pub fn compress(image_item: ImageItem, quality: u8) -> String {
    let image = image_item.get_image();
    let can_use_png = is_rgba8(&image_item.get_raw());
    let compress_image;
    let format_input;
    if can_use_png {
        // TODO
        compress_image = image;
        format_input = "data:image/png;base64,"
    } else {
        let out_format = ImageOutputFormat::Jpeg(quality);
        let mut buffer = Cursor::new(vec![]);
        image.write_to(&mut buffer, out_format).unwrap();
        let slice = buffer.get_ref().as_slice();
        compress_image = load_from_memory(slice).unwrap();
        format_input = "data:image/jpeg;base64,"
    }
    let bytes = compress_image.into_bytes();
    let b64 = general_purpose::STANDARD.encode(bytes);
    let rb64 = format!("${format_input},{}", b64.replace("\r\n", ""));
    rb64
}
