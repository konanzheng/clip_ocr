## 功能描述

rust OCR 识别小工具
1. 读取剪贴板中的图片内容，存到文件clip_ocr.png中
2. 调用tesseract 识别结果 存到result.txt
3. 读取result.txt 内容设置到剪贴板，并发送windows toast 通知。