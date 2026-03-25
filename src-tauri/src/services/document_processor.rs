use anyhow::Result;
use std::io::Read;

pub struct DocumentProcessor;

impl DocumentProcessor {
    /// Extract text from a document file
    pub fn extract_text(file_path: &str, doc_type: &str) -> Result<String> {
        match doc_type {
            "pdf" => Self::extract_pdf_text(file_path),
            "word" => Self::extract_word_text(file_path),
            _ => Err(anyhow::anyhow!("Unsupported document type")),
        }
    }

    /// Extract text from a PDF file
    fn extract_pdf_text(file_path: &str) -> Result<String> {
        let bytes = std::fs::read(file_path)?;
        let text = pdf_extract::extract_text_from_mem(&bytes)?;
        Ok(text)
    }

    /// Extract text from a Word document (docx)
    fn extract_word_text(file_path: &str) -> Result<String> {
        // Read docx file (which is essentially a zip file)
        let file = std::fs::File::open(file_path)?;
        let mut archive = zip::ZipArchive::new(file)?;

        // Extract word/document.xml
        let mut document_xml = String::new();
        if let Ok(mut doc_file) = archive.by_name("word/document.xml") {
            doc_file.read_to_string(&mut document_xml)?;
        }

        // Simple text extraction (remove XML tags)
        let text = Self::extract_text_from_xml(&document_xml);
        Ok(text)
    }

    /// Extract text content from XML by removing tags
    fn extract_text_from_xml(xml: &str) -> String {
        let mut result = String::new();
        let mut in_tag = false;

        for c in xml.chars() {
            match c {
                '<' => in_tag = true,
                '>' => in_tag = false,
                _ if !in_tag => result.push(c),
                _ => {}
            }
        }

        // Clean up extra whitespace
        result.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    /// Split text into paragraphs for embedding
    pub fn split_into_paragraphs(text: &str) -> Vec<String> {
        text.split("\n\n")
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}

impl Default for DocumentProcessor {
    fn default() -> Self {
        Self
    }
}