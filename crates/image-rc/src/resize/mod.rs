mod resize_item;

use std::io::Cursor;

use image::{imageops::FilterType, GenericImageView, ImageBuffer, Rgba};
use resize_item::ResizeItem;

use crate::{
    converter::Converter,
    utils::{logger, LoggerLevel},
};

pub fn resize(converter: &Converter, dist_width: u32, dist_height: u32) -> Converter {
    let (width, height) = converter.dimensions();
    let [w_f32, h_f32, dw_f32, dh_f32] = [width, height, dist_width, dist_height].map(|x| x as f32);

    let scale_x = w_f32 / dw_f32;
    let scale_y = h_f32 / dh_f32;

    let image = converter.load_image();
    let mut buffer = Vec::new();

    if scale_x > 1. && scale_y > 1. {
        let mut resize_buffer = ImageBuffer::new(dist_height, dist_height);

        let [scale_x_u32, scale_y_u32] = [scale_x, scale_y].map(|x| x as u32);

        for y in 0..dist_height {
            for x in 0..dist_width {
                let start_x = (x as f32 * scale_x).floor() as u32;
                let start_y = (y as f32 * scale_y).floor() as u32;
                let mut resize_item = ResizeItem::new();

                for j in 0..scale_y_u32 {
                    for i in 0..scale_x_u32 {
                        let rgba_u8 = image.get_pixel(start_x + i, start_y + j);
                        resize_item.push_pixel(&rgba_u8.0);
                    }
                }
                resize_buffer.put_pixel(x, y, Rgba(resize_item.average()));
            }
        }

        buffer = resize_buffer.to_vec();
    } else {
        image.resize(dist_width, dist_height, FilterType::CatmullRom);

        let _ = image
            .write_to(&mut Cursor::new(&mut buffer), converter.format())
            .expect(&logger(
                LoggerLevel::Error,
                "inside resize write buffer error",
            ));
    }

    return Converter::new_from_buffer(buffer);
}
