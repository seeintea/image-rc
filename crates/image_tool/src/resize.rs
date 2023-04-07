use image::{
    imageops::{resize, FilterType},
    DynamicImage, GenericImageView, ImageBuffer, Rgba,
};

struct RegionalPixel {
    r: u32,
    g: u32,
    b: u32,
    a: u32,
}

impl RegionalPixel {
    fn new(r: u32, g: u32, b: u32, a: u32) -> Self {
        RegionalPixel { r, g, b, a }
    }

    fn add_pixel(&mut self, pixel: &[u8; 4]) {
        let pixel_u32 = pixel.map(|x| x as u32);
        self.r += pixel_u32[0];
        self.g += pixel_u32[1];
        self.b += pixel_u32[2];
        self.a += pixel_u32[3];
    }

    fn get_regional_pixel(&self, count: u32) -> [u8; 4] {
        let mut pixels: [u8; 4] = [0, 0, 0, 0];
        pixels[0] = (self.r / count) as u8;
        pixels[1] = (self.g / count) as u8;
        pixels[2] = (self.b / count) as u8;
        pixels[3] = (self.a / count) as u8;
        pixels
    }
}

pub fn resize_image(
    image: DynamicImage,
    origin_width: u32,
    origin_height: u32,
    dist_width: u32,
    dist_height: u32,
) -> Vec<u8> {
    let scale_x = origin_width as f32 / dist_width as f32;
    let scale_y = origin_height as f32 / dist_height as f32;
    let mut resized_img = ImageBuffer::new(dist_width, dist_height);
    if scale_x > 1. && scale_y > 1. {
        let i_scale_x = scale_x as u32;
        let i_scale_y = scale_y as u32;
        for y in 0..dist_height {
            for x in 0..dist_width {
                let start_x = (x as f32 * scale_x).floor() as u32;
                let start_y = (y as f32 * scale_y).floor() as u32;
                let mut count = 0;
                let mut regional_pixel = RegionalPixel::new(0, 0, 0, 0);
                for j in 0..=i_scale_y {
                    for i in 0..=i_scale_x {
                        let pixel = image.get_pixel(start_x + i, start_y + j);
                        regional_pixel.add_pixel(&pixel.0);
                        count += 1;
                    }
                }
                resized_img.put_pixel(x, y, Rgba(regional_pixel.get_regional_pixel(count)))
            }
        }
    } else {
        resized_img = resize(&image, dist_width, dist_height, FilterType::CatmullRom);
    }
    resized_img.to_vec()
}
