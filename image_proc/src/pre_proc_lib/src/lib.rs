#[path = "../../img_proc/pre_process.rs"]
mod pre_process;
use image::{DynamicImage, ImageBuffer};
use pre_process::PreProc;
use std::ffi::{ c_void};
use std::slice;

#[repr(C)]
pub struct RustImage {
    pub width: u32,
    pub height: u32,
    pub channels: u32, // 新增：通道数 (1=灰度, 3=RGB, 4=RGBA)
    pub buffer: *const u8,
}

#[unsafe(no_mangle)]
pub extern "C" fn create_proc_handle(img: *const RustImage) -> *mut c_void {
    println!("in create_proc_handle");
    if img.is_null() {
        return std::ptr::null_mut();
    }
    println!("convert to rust image");
    let buf = unsafe { &*img };
    println!("convert to slice, width: {}, height: {}, channels: {}", buf.width, buf.height, buf.channels);
    
    let buffer_size = (buf.width * buf.height * buf.channels) as usize;
    let slice = unsafe { slice::from_raw_parts(buf.buffer, buffer_size) };
    
    println!("convert to image");
    let src_img = match buf.channels {
        1 => {
            // 灰度图像
            match ImageBuffer::from_raw(buf.width, buf.height, slice.to_vec()) {
                Some(img_buf) => DynamicImage::ImageLuma8(img_buf),
                None => {
                    println!("Failed to create grayscale image from raw data");
                    return std::ptr::null_mut();
                }
            }
        },
        3 => {
            // RGB图像
            match ImageBuffer::from_raw(buf.width, buf.height, slice.to_vec()) {
                Some(img_buf) => DynamicImage::ImageRgb8(img_buf),
                None => {
                    println!("Failed to create RGB image from raw data");
                    return std::ptr::null_mut();
                }
            }
        },
        4 => {
            // RGBA图像
            match ImageBuffer::from_raw(buf.width, buf.height, slice.to_vec()) {
                Some(img_buf) => DynamicImage::ImageRgba8(img_buf),
                None => {
                    println!("Failed to create RGBA image from raw data");
                    return std::ptr::null_mut();
                }
            }
        },
        _ => {
            println!("Unsupported number of channels: {}. Supported: 1 (Grayscale), 3 (RGB), 4 (RGBA)", buf.channels);
            return std::ptr::null_mut();
        }
    };
    println!("convert to pre_proc");
    let pre_proc = PreProc::new(src_img);
    println!("convert to box");
    Box::into_raw(Box::new(pre_proc)) as *mut c_void
}

/// 将图像转换为灰度图
/// 参数：
/// - handle: PreProc实例的句柄  
/// - buffer: 用于存储结果的RustImage结构体指针
/// 
/// 注意：此函数分配的内存需要使用free_image_buffer函数释放
#[unsafe(no_mangle)]
pub extern "C" fn to_gray(handle: *mut c_void, buffer: *mut RustImage) {
    if handle.is_null() {
        return;
    }

    if buffer.is_null() {
        return;
    }

    // 使用引用而不是取所有权
    let proc = unsafe { &*(handle as *mut PreProc) };
    let gray_img_buffer = match proc.to_gray() {
        Ok(img) => img,
        Err(e) => {
            println!("Failed to convert to gray: {}", e);
            return;
        }
    };
    
    let (width, height) = gray_img_buffer.dimensions();
    let raw_data = gray_img_buffer.into_raw();
    
    // 在堆上分配内存并复制数据
    let heap_data = raw_data.into_boxed_slice();
    let data_ptr = Box::into_raw(heap_data) as *const u8;
    
    let buf = unsafe { &mut*buffer };
    buf.width = width;
    buf.height = height;
    buf.channels = 1; // 灰度图像只有1个通道
    buf.buffer = data_ptr;

    // std::mem::forget();
}

/// 对图像进行二值化阈值处理
/// 参数：
/// - handle: PreProc实例的句柄
/// - buffer: 用于存储结果的RustImage结构体指针
/// - threshold: 阈值（0-255）
/// 
/// 注意：此函数分配的内存需要使用free_threshold_buffer函数释放
#[unsafe(no_mangle)]
pub extern "C" fn threshold(handle: *mut c_void, buffer: *mut RustImage, threshold: u8) {
    if handle.is_null() || buffer.is_null() {
        return;
    }

    let proc = unsafe { &*(handle as *mut PreProc) };
    let gray_img = proc.image().to_luma8();
    let threshold_buffer = match proc.threshold(gray_img, threshold) {
        Ok(img) => img,
        Err(e) => {
            println!("Failed to threshold: {}", e);
            return;
        }
    };

    let (width, height) = threshold_buffer.dimensions();
    
    let buf = unsafe { &mut*buffer };
    buf.width = width;
    buf.height = height;
    buf.channels = 1;
    buf.buffer =  threshold_buffer.as_ptr();

    // 此处保留内存的方式与 to_gray 不同,并同时实现了两种内存释放方式
    // 此处用作学习参考, 实际项目中, 只用其中一种能满足正常使用的方式即可
    std::mem::forget(threshold_buffer);

}

#[unsafe(no_mangle)]
pub extern "C" fn destroy_proc_handle(pre_proc: *mut c_void) {
    let _ = unsafe { Box::from_raw(pre_proc as *mut PreProc) };
}

/// 释放从to_gray等函数返回的图像数据内存（通过Box分配的内存）
/// 参数：
/// - buffer: 包含需要释放的内存指针的RustImage结构体
#[unsafe(no_mangle)]
pub extern "C" fn free_image_buffer(buffer: *const RustImage) {
    if buffer.is_null() {
        return;
    }
    
    let buf = unsafe { &*buffer };
    if buf.buffer.is_null() {
        return;
    }
    
    let buffer_size = (buf.width * buf.height * buf.channels) as usize;
    // 重新构造Box，然后让它自动释放
    let _heap_data = unsafe { 
        Box::from_raw(slice::from_raw_parts_mut(
            buf.buffer as *mut u8, 
            buffer_size
        ))
    };
    // heap_data在这里自动被释放
}

/// 释放从threshold函数返回的图像数据内存（通过forget分配的内存）
/// 参数：
/// - buffer: 包含需要释放的内存指针的RustImage结构体
/// 注意：此函数专门用于释放threshold函数中使用std::mem::forget的ImageBuffer内存
#[unsafe(no_mangle)]
pub extern "C" fn free_threshold_buffer(buffer: *const RustImage) {
    if buffer.is_null() {
        return;
    }
    
    let buf = unsafe { &*buffer };
    if buf.buffer.is_null() {
        return;
    }
    
    let buffer_size = (buf.width * buf.height * buf.channels) as usize;
    // 重新构造ImageBuffer来让它正确释放内存
    // 这里我们需要将原始指针重新包装成Vec，让它自动释放
    let vec_data = unsafe {
        Vec::from_raw_parts(
            buf.buffer as *mut u8,
            buffer_size,
            buffer_size
        )
    };
    // vec_data在这里自动被释放
    drop(vec_data);
}
