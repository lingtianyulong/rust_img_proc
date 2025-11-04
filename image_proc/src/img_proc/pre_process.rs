// 图像预处理模块, 包括灰度化, 二值化, 滤波, 边缘检测, 轮廓检测, 特征提取等
use image::{ DynamicImage, GrayImage};
use rayon::prelude::*;
use std::cell::{Ref, RefCell};
use std::io::{Error, ErrorKind};

#[repr(C)]
pub struct PreProc {
    src: RefCell<DynamicImage>, // RefCell 可以内部修改
}

#[allow(dead_code)]
impl PreProc {
    pub fn new(image: &DynamicImage) -> Self {
        Self {
            src: RefCell::new(image.to_owned()),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.src.borrow().width() == 0 || self.src.borrow().height() == 0
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

    #[allow(dead_code)]
    pub fn image(&self) -> Ref<'_, DynamicImage> {
        self.src.borrow()
    }
}
