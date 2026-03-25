use anyhow::Result;
use printpdf::*;
use std::fs::File;
use std::io::Read;

/// PDF 生成器：用于创建双语对照 PDF 文档
pub struct PdfGenerator;

impl PdfGenerator {
    /// 加载中文字体
    /// 尝试加载系统中的中文字体，按优先级尝试
    /// 注意：printpdf 不支持 .ttc（TrueType Collection）格式，只支持 .ttf 单字体文件
    fn load_cjk_font_bytes() -> Result<Vec<u8>> {
        // Windows 系统中文字体路径（只使用 .ttf 格式）
        // 注意：msyh.ttc、simsun.ttc 是 TrueType Collection 格式，printpdf 不支持
        let font_paths = [
            "C:\\Windows\\Fonts\\simhei.ttf",   // 黑体（.ttf 格式，推荐）
            "C:\\Windows\\Fonts\\simkai.ttf",   // 楷体
            "C:\\Windows\\Fonts\\STZHONGS.TTF", // 华文中宋
            "C:\\Windows\\Fonts\\STSONG.TTF",   // 华文宋体
            "C:\\Windows\\Fonts\\STKAITI.TTF",  // 华文楷体
        ];

        for path in &font_paths {
            if std::path::Path::new(path).exists() {
                match File::open(path) {
                    Ok(mut font_file) => {
                        let mut font_bytes = Vec::new();
                        match font_file.read_to_end(&mut font_bytes) {
                            Ok(_) => {
                                println!("成功加载中文字体: {}", path);
                                return Ok(font_bytes);
                            }
                            Err(e) => {
                                println!("读取字体文件 {} 失败: {}, 尝试下一个", path, e);
                                continue;
                            }
                        }
                    }
                    Err(e) => {
                        println!("无法打开字体文件 {}: {}", path, e);
                        continue;
                    }
                }
            }
        }

        Err(anyhow::anyhow!(
            "未找到可用的中文字体。请确保系统安装了微软雅黑、黑体、宋体或楷体字体"
        ))
    }

    /// 生成双语对照 PDF
    /// 段落格式：中文段落后紧跟英文翻译
    pub fn generate_bilingual_pdf(
        output_path: &str,
        paragraphs: &[(String, String)], // (中文, 英文)
    ) -> Result<()> {
        // A4 纸张大小 (210mm x 297mm)
        let (width, height) = (Mm(210.0), Mm(297.0));
        let margin = Mm(20.0);

        // 创建 PDF 文档（带初始页面和图层）
        let (doc, page1, layer1) = PdfDocument::new(
            "DocTranslate Output",
            width,
            height,
            "Layer 1",
        );

        // 尝试加载中文字体
        let font_bytes = Self::load_cjk_font_bytes()?;
        let cursor = std::io::Cursor::new(font_bytes);
        let font = doc.add_external_font(cursor)
            .map_err(|e| anyhow::anyhow!("添加外部字体失败: {}", e))?;

        // 跟踪当前页面和图层索引
        let mut current_page = page1;
        let mut current_layer_id = layer1;

        // 字体大小和行高
        let font_size = 10.0;
        let line_height = 14.0;

        // 起始 Y 位置（从顶部开始，printpdf 坐标系从左下角开始）
        let mut y = height - margin;

        for (chinese, english) in paragraphs {
            // 中文段落换行处理
            let chinese_lines = Self::wrap_text(chinese, 40);
            let english_lines = Self::wrap_text(english, 60);

            // 计算当前段落需要的总高度
            let total_lines = chinese_lines.len() + english_lines.len();
            let paragraph_height = Mm(total_lines as f32 * line_height + 5.0 + 10.0);

            // 检查是否需要新页
            if y < margin + paragraph_height {
                // 如果剩余空间不足以放下整个段落，创建新页面
                if y < margin + Mm(40.0) {
                    let (new_page, new_layer) = doc.add_page(width, height, "Layer 1");
                    current_page = new_page;
                    current_layer_id = new_layer;
                    y = height - margin;
                }
            }

            // 写入中文段落
            for line in &chinese_lines {
                // 检查是否需要换页
                if y < margin + Mm(10.0) {
                    let (new_page, new_layer) = doc.add_page(width, height, "Layer 1");
                    current_page = new_page;
                    current_layer_id = new_layer;
                    y = height - margin;
                }

                // 获取当前图层用于绘制（关键修复：每次绘制前获取正确的图层）
                let current_layer = doc.get_page(current_page).get_layer(current_layer_id);
                current_layer.use_text(line, font_size, margin, y, &font);
                y -= Mm(line_height);
            }

            // 中英文段落间距
            y -= Mm(5.0);

            // 写入英文段落
            for line in &english_lines {
                // 检查是否需要换页
                if y < margin + Mm(10.0) {
                    let (new_page, new_layer) = doc.add_page(width, height, "Layer 1");
                    current_page = new_page;
                    current_layer_id = new_layer;
                    y = height - margin;
                }

                // 获取当前图层用于绘制
                let current_layer = doc.get_page(current_page).get_layer(current_layer_id);
                current_layer.use_text(line, font_size, margin, y, &font);
                y -= Mm(line_height);
            }

            // 段落之间的额外间距
            y -= Mm(10.0);
        }

        // 保存文档
        doc.save(&mut std::io::BufWriter::new(File::create(output_path)?))?;

        Ok(())
    }

    /// 文本换行处理
    fn wrap_text(text: &str, chars_per_line: usize) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current_line = String::new();
        let mut char_count = 0;

        for ch in text.chars() {
            if ch == '\n' {
                if !current_line.is_empty() {
                    lines.push(current_line.clone());
                    current_line.clear();
                    char_count = 0;
                } else {
                    lines.push(String::new());
                }
            } else if ch == '\r' {
                // 忽略回车符
                continue;
            } else {
                current_line.push(ch);
                char_count += 1;

                if char_count >= chars_per_line {
                    lines.push(current_line.clone());
                    current_line.clear();
                    char_count = 0;
                }
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        if lines.is_empty() {
            lines.push(String::new());
        }

        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_text() {
        let text = "这是一段测试文本";
        let lines = PdfGenerator::wrap_text(text, 4);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "这是一段");
        assert_eq!(lines[1], "测试文本");
    }

    #[test]
    fn test_wrap_text_with_newline() {
        let text = "第一行\n第二行";
        let lines = PdfGenerator::wrap_text(text, 10);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "第一行");
        assert_eq!(lines[1], "第二行");
    }

    #[test]
    fn test_wrap_text_empty() {
        let lines = PdfGenerator::wrap_text("", 10);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], "");
    }
}