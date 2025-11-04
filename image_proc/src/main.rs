use image::open;
mod conv_color;
mod img_proc;
use conv_color::conv_color::ConvterColor;
use img_proc::pre_process::PreProc;

fn main() {
    let _i = 42;
    let image = open("D:\\1.jpg").unwrap();
    let pre_proc = PreProc::new(&image);

    let gray_image = match ConvterColor::to_gray(&image) {
        Ok(gray) => {
            println!("转换成功");
            gray.save("gray.png").unwrap();
            gray // 返回灰度图像
        }
        Err(e) => {
            println!("转换失败: {}", e);
            return; // 提前退出
        }
    };

    let binary_image = match pre_proc.threshold(gray_image, 128) {
        Ok(binary) => binary,
        Err(e) => {
            println!("二值化失败: {}", e);
            return; // 提前退出
        }
    };

    binary_image
        .save("binary.png")
        .expect("Failed to save image");
}
