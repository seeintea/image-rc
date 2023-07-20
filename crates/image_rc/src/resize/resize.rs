use image::{imageops, DynamicImage, GenericImageView, ImageBuffer, Rgba};

use super::resize_raws::ResizeRaws;

pub fn resize_image(
    image: DynamicImage,
    origin_width: u32,
    origin_height: u32,
    dist_width: u32,
    dist_height: u32,
) -> DynamicImage {
    let mut resize_item_buffer = ImageBuffer::new(dist_width, dist_height);

    let scale_x = origin_width as f32 / dist_height as f32;
    let scale_y = origin_height as f32 / dist_height as f32;

    if scale_x > 1. && scale_y > 1. {
        let scale_x_u32 = scale_x as u32;
        let scale_y_u32 = scale_y as u32;

        for y in 0..dist_height {
            for x in 0..dist_width {
                let start_x = (x as f32 * scale_x).floor() as u32;
                let start_y = (y as f32 * scale_y).floor() as u32;
                let mut resize_raws = ResizeRaws::new();

                for j in 0..scale_y_u32 {
                    for i in 0..scale_x_u32 {
                        let rgba_u8 = image.get_pixel(start_x + i, start_y + j);
                        resize_raws.add_item(&rgba_u8.0);
                    }
                }
                resize_item_buffer.put_pixel(x, y, Rgba(resize_raws.get_average()));
            }
        }
    } else {
        resize_item_buffer = imageops::resize(
            &image,
            dist_width,
            dist_height,
            imageops::FilterType::CatmullRom,
        );
    }

    DynamicImage::ImageRgba8(resize_item_buffer)
}
