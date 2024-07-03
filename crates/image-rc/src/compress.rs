use image::{
    codecs::{
        jpeg::JpegEncoder,
        png::{CompressionType, FilterType, PngEncoder},
    },
    DynamicImage,
};

use crate::{
    converter::Converter,
    utils::{logger, LoggerLevel},
};

pub fn compress(converter: &Converter, quality: u8) -> Converter {
    if quality == 0 || quality >= 100 {
        return converter.clone();
    }

    let origin_image = converter.load_image();
    let color_type = origin_image.color();

    let buffer: Vec<u8>;

    if color_type.has_alpha() {
        buffer = compress_png(origin_image);
    } else {
        buffer = compress_jpeg(origin_image, quality);
    }

    Converter::new_from_buffer(buffer)
}

fn compress_png(image: DynamicImage) -> Vec<u8> {
    let mut buffer = Vec::new();
    let encoder =
        PngEncoder::new_with_quality(&mut buffer, CompressionType::Best, FilterType::Adaptive);

    image
        .write_with_encoder(encoder)
        .expect(&logger(LoggerLevel::Error, "write with jpegEncoder failed"));

    buffer
}

fn compress_jpeg(image: DynamicImage, quality: u8) -> Vec<u8> {
    let mut buffer = Vec::new();
    let encoder = JpegEncoder::new_with_quality(&mut buffer, quality);

    image
        .write_with_encoder(encoder)
        .expect(&logger(LoggerLevel::Error, "write with jpegEncoder failed"));

    buffer
}
