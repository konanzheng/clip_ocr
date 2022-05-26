## 功能描述

rust OCR 识别小工具
1. 读取剪贴板中的图片内容，存到文件clip_ocr.png中
2. 调用tesseract 识别结果 存到result.txt
3. 读取result.txt 内容设置到剪贴板，并发送windows toast 通知。


## 使用说明
1. 安装tesseract 和安装tesseract-langpack-chi_sim 然后设置好path 保证可以在命令行中调用 ``` tesseract xxx.png result -l chi_sim```
2. 构建或下载clip_ocr.exe 并生成一个快捷方式 设置快捷键
3. 日常使用 使用微信```ALT+A``` qq ```CTRL+ALT+A ```或系统截屏 ```WIN+SHIFT+S``` 截屏后 按下clip_ocr快捷键 等会出弹出框就是识别好了 可以粘贴了