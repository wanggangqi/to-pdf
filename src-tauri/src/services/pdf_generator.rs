use anyhow::Result;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

/// PDF 生成器：用于创建双语对照 PDF 文档
pub struct PdfGenerator;

impl PdfGenerator {
    /// 生成双语对照 PDF
    /// 段落格式：中文段落后紧跟英文翻译
    /// 注意：中文支持需要嵌入字体文件，这里使用简化的实现方案
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

        // 使用内置 Helvetica 字体（用于英文）
        let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;

        // 获取当前图层用于绘制
        let current_layer = doc.get_page(page1).get_layer(layer1);

        // 字体大小和行高
        let font_size = 10.0;
        let line_height = 14.0;

        // 起始 Y 位置（从顶部开始，printpdf 坐标系从左下角开始）
        let mut y = height - margin;

        for (chinese, english) in paragraphs {
            // 检查是否需要新页（预留 40mm 空间）
            if y < margin + Mm(40.0) {
                // 创建新页面
                let (_new_page, _new_layer) = doc.add_page(width, height, "Layer 1");
                y = height - margin;
            }

            // 中文段落 - 使用内置字体（注意：可能无法正确显示中文）
            // 实际项目中需要嵌入中文字体
            let chinese_lines = Self::wrap_text(chinese, 40);
            for line in &chinese_lines {
                current_layer.use_text(line, font_size, margin, y, &font);
                y -= Mm(line_height);
            }
            y -= Mm(5.0); // 段落间距

            // 英文段落
            let english_lines = Self::wrap_text(english, 60);
            for line in &english_lines {
                current_layer.use_text(line, font_size, margin, y, &font);
                y -= Mm(line_height);
            }
            y -= Mm(10.0); // 段落之间的额外间距
        }

        // 保存文档
        doc.save(&mut BufWriter::new(File::create(output_path)?))?;

        Ok(())
    }

    /// 估算文本所需行数
    #[allow(dead_code)]
    fn estimate_lines(text: &str, chars_per_line: usize) -> usize {
        let char_count = text.chars().count();
        if char_count == 0 {
            return 0;
        }
        (char_count + chars_per_line - 1) / chars_per_line
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
    fn test_estimate_lines() {
        assert_eq!(PdfGenerator::estimate_lines("", 10), 0);
        assert_eq!(PdfGenerator::estimate_lines("hello", 10), 1);
        assert_eq!(PdfGenerator::estimate_lines("hello world test", 10), 2);
    }

    #[test]
    fn test_wrap_text() {
        let text = "这是一段测试文本";
        let lines = PdfGenerator::wrap_text(text, 4);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "这是一段");
        assert_eq!(lines[1], "测试文本");
    }
}