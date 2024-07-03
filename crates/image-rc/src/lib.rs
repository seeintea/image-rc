mod compress;
mod converter;
mod resize;
mod utils;

use converter::Converter;

pub fn load_from_base64(data: &str) -> Converter {
    Converter::new_from_base64(data)
}

pub fn load_form_buffer(data: Vec<u8>) -> Converter {
    Converter::new_from_buffer(data)
}

pub fn rc_resize(converter: &Converter, dist_width: u32, dist_height: u32) -> Converter {
    resize::resize(converter, dist_width, dist_height)
}

pub fn rc_compress(converter: &Converter, quality: u8) -> Converter {
    compress::compress(converter, quality)
}
