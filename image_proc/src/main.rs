use image::{ open };
mod img_proc;
use img_proc::pre_process::PreProc;

fn main() {
    let image = open("D:\\1.jpg").unwrap();
    let pre_proc = PreProc::new(image);

    let _gray_image = match pre_proc.to_gray() {
        Ok(gray) => {
            println!("转换成功");
            gray.save("gray.png").unwrap();
            gray  // 返回灰度图像
        }
        Err(e) => {
            println!("转换失败: {}", e);
            return;  // 提前退出
        }
    };

}
