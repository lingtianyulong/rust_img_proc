// 图像预处理模块, 包括灰度化, 二值化, 滤波, 边缘检测, 轮廓检测, 特征提取等
use image::{ColorType, DynamicImage, GrayImage};
use rayon::prelude::*;
use std::cell::{Ref, RefCell};
use std::io::{Error, ErrorKind};

#[repr(C)]
pub struct PreProc {
    src: RefCell<DynamicImage>, // RefCell 可以内部修改
}

impl PreProc {
    pub fn new(image: DynamicImage) -> Self {
        Self {
            src: RefCell::new(image),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.src.borrow().width() == 0 || self.src.borrow().height() == 0
    }

    // 返回引用，零拷贝，但调用者需要在 PreProc 实例存在期间使用结果
    pub fn to_gray(&self) -> Result<GrayImage, Error> {
        if self.is_empty() {
            return Err(Error::new(ErrorKind::InvalidData, "Image is empty"));
        }

        let color_type = self.src.borrow().color();
        match color_type {
            ColorType::Rgb8 | ColorType::Rgba8 => Ok(self.src.borrow().to_luma8()),
            ColorType::L8 | ColorType::La8 => Ok(self.src.borrow().to_luma8()),
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Image is not RGB or RGBA or Luma8",
                ));
            }
        }
    }

    pub fn threshold(&self, gray: GrayImage, threshold: u8) -> Result<GrayImage, Error> {
        if self.is_empty() {
            return Err(Error::new(ErrorKind::InvalidData, "Image is empty"));
        }

        let (width, height) = gray.dimensions();
        let mut buffer = gray.into_raw();
        // 并行处理
        buffer.par_iter_mut().for_each(|pixel| {
            *pixel = if *pixel > threshold { 255 } else { 0 };
        });

        Ok(GrayImage::from_raw(width, height, buffer)
            .expect("Failed to create image from raw data"))
    }

    pub fn image(&self) -> Ref<DynamicImage> {
        self.src.borrow()
    }
}
