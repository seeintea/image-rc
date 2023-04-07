use image::{
    imageops::{resize as image_resize, FilterType},
    DynamicImage, GenericImageView, ImageBuffer, Rgba,
};

struct RegionalPixels {
    r: u32,
    g: u32,
    b: u32,
    a: u32,
    count: u32,
}

impl RegionalPixels {
    fn new(r: u32, g: u32, b: u32, a: u32) -> Self {
        RegionalPixels {
            r,
            g,
            b,
            a,
            count: 0,
        }
    }

    fn add_pixel(&mut self, pixel: &[u8; 4]) {
        let pixel_u32 = pixel.map(|x| x as u32);
        self.r += pixel_u32[0];
        self.g += pixel_u32[1];
        self.b += pixel_u32[2];
        self.a += pixel_u32[3];
        self.count += 1;
    }

    fn get_average_pixel(&self) -> [u8; 4] {
        let mut pixels: [u8; 4] = [0, 0, 0, 0];
        let count = self.count;
        pixels[0] = (self.r / count) as u8;
        pixels[1] = (self.g / count) as u8;
        pixels[2] = (self.b / count) as u8;
        pixels[3] = (self.a / count) as u8;
        pixels
    }
}

pub fn resize(
    image: DynamicImage,
    origin_width: u32,
    origin_height: u32,
    dist_width: u32,
    dist_height: u32,
) -> DynamicImage {
    let scale_x = origin_width as f32 / dist_width as f32;
    let scale_y = origin_height as f32 / dist_height as f32;
    let mut resize_buffer = ImageBuffer::new(dist_width, dist_height);
    if scale_x > 1. && scale_y > 1. {
        let i_scale_x = scale_x as u32;
        let i_scale_y = scale_y as u32;
        for y in 0..dist_height {
            for x in 0..dist_width {
                let start_x = (x as f32 * scale_x).floor() as u32;
                let start_y = (y as f32 * scale_y).floor() as u32;
                let mut regional_pixels = RegionalPixels::new(0, 0, 0, 0);
                for j in 0..=i_scale_y {
                    for i in 0..=i_scale_x {
                        let pixel = image.get_pixel(start_x + i, start_y + j);
                        regional_pixels.add_pixel(&pixel.0);
                    }
                }
                resize_buffer.put_pixel(x, y, Rgba(regional_pixels.get_average_pixel()))
            }
        }
    } else {
        resize_buffer = image_resize(&image, dist_width, dist_height, FilterType::CatmullRom);
    }
    DynamicImage::ImageRgba8(resize_buffer)
}
