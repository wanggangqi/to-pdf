use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::models::ProviderConfig;
use super::ai_client::{chat, Message};

/// 术语表：中文 -> 英文映射（共享状态）
pub type TermGlossary = Arc<RwLock<HashMap<String, String>>>;

pub struct Translator {
    provider: ProviderConfig,
    glossary: TermGlossary,
}

impl Translator {
    pub fn new(provider: ProviderConfig) -> Self {
        Self {
            provider,
            glossary: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 创建带有共享术语表的实例（用于并发翻译）
    pub fn with_glossary(provider: ProviderConfig, glossary: TermGlossary) -> Self {
        Self { provider, glossary }
    }

    /// 获取术语表引用（用于共享）
    pub fn glossary(&self) -> TermGlossary {
        self.glossary.clone()
    }

    /// 翻译单个段落
    pub async fn translate_paragraph(&self, text: &str) -> Result<String> {
        let glossary = self.glossary.read().await;
        let glossary_text = if glossary.is_empty() {
            String::new()
        } else {
            let terms: Vec<String> = glossary
                .iter()
                .map(|(k, v)| format!("{} -> {}", k, v))
                .collect();
            format!("\n\n术语参考：\n{}", terms.join("\n"))
        };
        drop(glossary); // 释放读锁

        let system_prompt = format!(
            "你是一个专业的中英翻译助手。请将以下中文翻译成英文。\
             保持专业和准确。{}\
             \n\n只返回翻译结果，不要添加任何解释。",
            glossary_text
        );

        let messages = vec![
            Message {
                role: "system".to_string(),
                content: system_prompt,
            },
            Message {
                role: "user".to_string(),
                content: text.to_string(),
            },
        ];

        let result = chat(&self.provider, messages).await?;

        // 更新术语表（简化实现，暂不自动提取）
        // self.update_glossary(text, &result).await;

        Ok(result)
    }

    /// 批量翻译段落（并发，共享术语表）
    pub async fn translate_paragraphs(
        &self,
        paragraphs: &[String],
        concurrency: usize,
    ) -> Result<Vec<String>> {
        use tokio::sync::Semaphore;

        let semaphore = Arc::new(Semaphore::new(concurrency));
        let glossary = self.glossary.clone(); // 共享术语表
        let provider = self.provider.clone();
        let mut handles = vec![];

        for para in paragraphs {
            let permit = semaphore.clone().acquire_owned().await?;
            let glossary_clone = glossary.clone();
            let provider_clone = provider.clone();
            let para = para.clone();

            // 使用共享术语表创建翻译器
            let translator = Self::with_glossary(provider_clone, glossary_clone);

            let handle = tokio::spawn(async move {
                let _permit = permit;
                translator.translate_paragraph(&para).await
            });
            handles.push(handle);
        }

        let mut results = vec![];
        for handle in handles {
            results.push(handle.await??);
        }

        Ok(results)
    }

    pub async fn add_term(&self, chinese: String, english: String) {
        let mut glossary = self.glossary.write().await;
        glossary.insert(chinese, english);
    }
}