use base64::{engine::general_purpose, Engine};
use image::{DynamicImage, ImageBuffer, Rgba};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct ImageItem {
    width: u32,
    height: u32,
    raw: Vec<u8>,
}

impl ImageItem {
    pub fn new(width: u32, height: u32, raw: Vec<u8>) -> Self {
        ImageItem { width, height, raw }
    }

    pub fn load_from_base64(chars: &str) -> Self {
        let mut data = chars;
        let replace_regex = Regex::new(r"data:image\/(jpeg|png);base64,").unwrap();
        let mut data_replace;
        if replace_regex.is_match(data) {
            data_replace = replace_regex.replace(data, "");
            data = data_replace.to_mut();
        }
        let data_vec = general_purpose::STANDARD.decode(data).unwrap();
        let data_slice = data_vec.as_slice();
        let image_item = image::load_from_memory(data_slice).unwrap();
        ImageItem {
            width: image_item.width(),
            height: image_item.height(),
            raw: image_item.clone().into_bytes(),
        }
    }

    pub fn get_image(&self) -> DynamicImage {
        let image_buffer =
            ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(self.width, self.height, self.raw.clone())
                .unwrap();
        DynamicImage::ImageRgba8(image_buffer)
    }

    pub fn get_raw(&self) -> &Vec<u8> {
        &self.raw
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_image_default() {
        let raw = vec![0, 0, 0, 0];
        let compare = raw.clone();
        let image_item = ImageItem::new(1, 1, raw);
        assert_eq!(compare, image_item.get_image().into_bytes());
    }

    #[test]
    fn create_image_base64() {
        let chars = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAAAXNSR0IArs4c6QAAAAtJREFUGFdjYAACAAAFAAGq1chRAAAAAElFTkSuQmCC";
        let image_item = ImageItem::load_from_base64(chars);
        let image_object = image_item.get_image();
        assert_eq!(image_object.width(), 1);
        assert_eq!(image_object.height(), 1);
        assert_eq!(image_object.into_bytes(), vec![0, 0, 0, 0]);
    }
}
