use base64::{engine::general_purpose, Engine as _};
use image::{
    guess_format, load_from_memory_with_format, DynamicImage, GenericImageView, ImageFormat,
};
use regex::Regex;
use std::borrow::Cow;

use crate::utils::{logger, LoggerLevel};

#[derive(Debug, Clone)]
pub struct Converter {
    width: u32,
    height: u32,
    buffer: Vec<u8>,
    format: ImageFormat,
}

impl Converter {
    pub fn new(width: u32, height: u32, buffer: Vec<u8>) -> Self {
        let format = guess_format(&buffer)
            .expect(&logger(LoggerLevel::Error, "failed to parse image format"));

        Self {
            width,
            height,
            buffer,
            format,
        }
    }

    pub fn new_from_buffer(buffer: Vec<u8>) -> Self {
        let buffer = buffer.as_slice();

        let error_message = &logger(LoggerLevel::Error, "failed to load image from buffer");
        let format = guess_format(buffer).expect(error_message);
        let image = load_from_memory_with_format(buffer, format).expect(error_message);
        let (width, height) = image.dimensions();

        Self {
            width,
            height,
            buffer: buffer.to_vec(),
            format,
        }
    }

    pub fn new_from_base64(data: &str) -> Self {
        let data: String = remove_base64_prefix(data);

        let buffer = general_purpose::STANDARD
            .decode(data.as_str())
            .expect(&logger(LoggerLevel::Error, "failed to load base64"));
        let buffer = buffer.as_slice();

        let error_message = &logger(LoggerLevel::Error, "failed to load image from base64");
        let format = guess_format(buffer).expect(error_message);
        let image = load_from_memory_with_format(buffer, format).expect(error_message);
        let (width, height) = image.dimensions();

        Self {
            width,
            height,
            buffer: buffer.to_vec(),
            format,
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn buffer(&self) -> Vec<u8> {
        self.buffer.clone()
    }

    pub fn format(&self) -> ImageFormat {
        self.format
    }

    pub fn load_image(&self) -> DynamicImage {
        let image = load_from_memory_with_format(self.buffer.as_slice(), self.format)
            .expect(&logger(LoggerLevel::Error, "Converter::load_image failed"));

        image
    }
}

fn remove_base64_prefix(data: &str) -> String {
    let remove = Regex::new(r"data:image\/(jpeg|png);base64,").unwrap();
    let mut result = Cow::from(data);
    if remove.is_match(data) {
        result = remove.replace(data, "");
    }

    result.into_owned().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_base64_prefix() {
        let data = "data:image/png;base64, image/png";
        assert_eq!(" image/png", remove_base64_prefix(data));
        let data = "data:image/jpeg;base64, image/jpeg";
        assert_eq!(" image/jpeg", remove_base64_prefix(data));
    }

    // 2x1.png create by canvas
    // r#"const canvas = document.createElement("canvas")
    // canvas.width = 2;
    // canvas.height = 1;
    // const ctx = canvas.getContext("2d")
    // ctx.fillStyle = "green"
    // ctx.fillRect(0, 0, 2, 2)
    // ctx.fillStyle = "blue"
    // ctx.fillRect(0, 0, 1, 1)"

    #[test]
    fn test_converter_new_from_buffer() {
        let buffer: Vec<u8> = vec![
            137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 2, 0, 0, 0, 1,
            8, 6, 0, 0, 0, 244, 34, 127, 138, 0, 0, 0, 1, 115, 82, 71, 66, 0, 174, 206, 28, 233, 0,
            0, 0, 17, 73, 68, 65, 84, 24, 87, 99, 100, 96, 248, 255, 159, 161, 129, 145, 1, 0, 12,
            137, 2, 129, 247, 152, 230, 182, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
        ];

        let converter = Converter::new_from_buffer(buffer.clone());

        assert_eq!(converter.dimensions(), (2, 1));
        assert_eq!(converter.format(), ImageFormat::Png);
        assert_eq!(converter.buffer(), buffer);

        let image = converter.load_image();
        assert_eq!(image.dimensions(), (2, 1));
    }

    #[test]
    fn test_converter_new_from_base64() {
        let data = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAIAAAABCAYAAAD0In+KAAAAAXNSR0IArs4c6QAAABFJREFUGFdjZGD4/5+hgZEBAAyJAoH3mOa2AAAAAElFTkSuQmCC";
        let converter = Converter::new_from_base64(data);

        assert_eq!(converter.dimensions(), (2, 1));
        assert_eq!(converter.format(), ImageFormat::Png);
        assert_eq!(
            converter.buffer(),
            vec![
                137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 2, 0, 0, 0,
                1, 8, 6, 0, 0, 0, 244, 34, 127, 138, 0, 0, 0, 1, 115, 82, 71, 66, 0, 174, 206, 28,
                233, 0, 0, 0, 17, 73, 68, 65, 84, 24, 87, 99, 100, 96, 248, 255, 159, 161, 129,
                145, 1, 0, 12, 137, 2, 129, 247, 152, 230, 182, 0, 0, 0, 0, 73, 69, 78, 68, 174,
                66, 96, 130,
            ]
        );

        let image = converter.load_image();
        assert_eq!(image.dimensions(), (2, 1));
    }
}
