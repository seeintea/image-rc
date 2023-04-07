pub mod resize;
pub mod utils;

pub fn resize_image_by_image_data(
    raw: Vec<u8>,
    origin_width: u32,
    origin_height: u32,
    dist_width: u32,
    dist_height: u32,
) -> Vec<u8> {
    let image = utils::load_image_by_image_data(raw, origin_width, origin_height);
    resize::resize_image(image, origin_width, origin_height, dist_width, dist_height)
}
