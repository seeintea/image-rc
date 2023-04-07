use image::{DynamicImage, ImageBuffer, Rgba};

pub fn load_image_by_image_data(raw: Vec<u8>, width: u32, height: u32) -> DynamicImage {
    let image_buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(width, height, raw).unwrap();
    DynamicImage::ImageRgba8(image_buffer)
}