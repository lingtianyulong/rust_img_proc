use image::{ColorType, DynamicImage, GenericImageView, GrayImage, Rgb, RgbImage};
// use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
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

    /// RGB 到 HSV 的转换
    fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
        let r = r as f32 / 255.0;
        let g = g as f32 / 255.0;
        let b = b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * ((g - b) / delta).rem_euclid(6.0)
        } else if max == g {
            60.0 * ((b - r) / delta + 2.0)
        } else {
            60.0 * ((r - g) / delta + 4.0)
        };

        let s = if max == 0.0 { 0.0 } else { delta / max };
        let v = max;

        (h, s, v)
    }

    pub fn to_hsv_image(image: &DynamicImage) -> Result<DynamicImage, Error> {
        if image.width() == 0 || image.height() == 0 {
            return Err(Error::new(ErrorKind::InvalidData, "Image is empty"));
        }

        let is_rgb = matches!(image.color(), ColorType::Rgb8 | ColorType::Rgba8);
        if !is_rgb {
            return Err(Error::new(ErrorKind::InvalidData, "Unsupported color type"));
        }

        let mut hsv_img = RgbImage::new(image.width(), image.height());
        let (width, height) = (image.width(), image.height());
        use rayon::prelude::*;

        // 并行计算HSV值
        let pixels: Vec<_> = (0..height)
            .into_par_iter()
            .flat_map(|y| {
                (0..width).into_par_iter().map(move |x| {
                    let pixel = image.get_pixel(x, y);
                    let r = pixel[0];
                    let g = pixel[1];
                    let b = pixel[2];
                    let (h, s, v) = ConvterColor::rgb_to_hsv(r, g, b);
                    (
                        x,
                        y,
                        Rgb([
                            (h as f32 / 360.0 * 255.0) as u8,
                            (s as f32 * 255.0) as u8,
                            (v as f32 * 255.0) as u8,
                        ]),
                    )
                })
            })
            .collect();

        // 将计算结果写入图像
        // for (x, y, hsv_pixel) in pixels {
        //     hsv_img.put_pixel(x, y, hsv_pixel);
        // }

        let buf = hsv_img.as_mut(); // 得到底层 buffer: &mut [u8]
        let stride = 3; // RGB 每像素 3 字节

        for (x, y, px) in pixels {
            let idx = ((y * width + x) * stride) as usize;
            buf[idx..idx + 3].copy_from_slice(&px.0);
        }

        Ok(DynamicImage::ImageRgb8(hsv_img))
    }
}
