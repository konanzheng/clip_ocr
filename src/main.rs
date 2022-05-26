use clipboard_win::{Clipboard, formats, Getter, set_clipboard};
use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::process::{Command};
use winrt_notification::{Duration, Sound, Toast};
fn main() {
  
    let _clip = Clipboard::new_attempts(10).expect("Open clipboard");
    let mut image = Vec::new();
    let length = formats::Bitmap.read_clipboard(&mut image).expect("Read sample");
    // println!("读取到剪贴板图片大小:{}", length);
    if length>0 {
        let mut file = File::create("clipboard_ocr.png").expect("create failed");
        file.write_all(&image).expect("将剪贴板内容写入图片文件");
        // println!("写入剪贴板图片:{}",file);
        // 调用tesseract 识别
        Command::new("tesseract").args(["clipboard_ocr.png","result","-l","chi_sim"]).output().unwrap();
        let mut contents = std::fs::read_to_string("result.txt").unwrap();
        contents = contents.replace("\n\n", "\n");
        set_clipboard(formats::Unicode,&contents).expect("To set clipboard");
        Toast::new(Toast::POWERSHELL_APP_ID)
        .title("OCR结果")
        .text1(&contents)
        .image(&Path::new("./clipboard_ocr.png"), "the ocr image")
        .sound(Some(Sound::SMS))
        .duration(Duration::Short)
        .show()
        .expect("unable to toast");
    }

}
