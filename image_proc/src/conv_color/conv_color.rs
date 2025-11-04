use image::{ColorType, DynamicImage, GrayImage};
use std::io::{Error, ErrorKind};

#[allow(dead_code)]
pub struct ConvterColor;

#[allow(dead_code)]
impl ConvterColor {
    pub fn to_gray(image: &DynamicImage) -> Result<GrayImage, Error> {
        if image.width() == 0 || image.height() == 0 {
            return Err(Error::new(ErrorKind::InvalidData, "Image is empty"));
        }

        let color_type = image.color();
        match color_type {
            ColorType::Rgb8 | ColorType::Rgba8 => Ok(image.to_luma8()),
            ColorType::L8 | ColorType::La8 => Ok(image.to_luma8()),
            _ => Err(Error::new(ErrorKind::InvalidData, "Unsupported color type")),
        }
    }
}
