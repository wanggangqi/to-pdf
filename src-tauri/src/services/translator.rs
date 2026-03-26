use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Semaphore;
use crate::models::ProviderConfig;
use super::ai_client::{chat, Message};

/// 翻译进度回调
pub type TranslateProgressCallback = Arc<dyn Fn(usize, usize) + Send + Sync>;

pub struct Translator {
    provider: ProviderConfig,
}

impl Translator {
    pub fn new(provider: ProviderConfig) -> Self {
        Self { provider }
    }

    /// 翻译单个段落
    pub async fn translate_paragraph(&self, text: &str) -> Result<String> {
        let system_prompt = "你是一个专业的中英翻译助手。请将以下中文翻译成英文。\
             保持专业和准确。\n\n只返回翻译结果，不要添加任何解释。";

        let messages = vec![
            Message {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: text.to_string(),
            },
        ];

        chat(&self.provider, messages).await
    }

    /// 批量翻译段落（并发，带进度回调，容错处理）
    /// - paragraphs: 段落列表
    /// - concurrency: 并发数（建议 10-15）
    /// - on_progress: 进度回调 (completed, total)
    ///
    /// 注意：如果某段落翻译失败（如内容审核），会使用原文代替
    pub async fn translate_paragraphs_with_progress(
        provider: ProviderConfig,
        paragraphs: &[String],
        concurrency: usize,
        on_progress: Option<TranslateProgressCallback>,
    ) -> Result<Vec<String>> {
        let semaphore = Arc::new(Semaphore::new(concurrency));
        let total = paragraphs.len();
        let completed = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let failed_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));

        let mut handles = vec![];

        for (idx, para) in paragraphs.iter().enumerate() {
            let permit = semaphore.clone().acquire_owned().await?;
            let provider_clone = provider.clone();
            let para = para.clone();
            let completed_clone = completed.clone();
            let failed_clone = failed_count.clone();
            let on_progress_clone = on_progress.clone();

            let handle = tokio::spawn(async move {
                let _permit = permit;

                // 创建临时翻译器
                let translator = Self::new(provider_clone);
                let result = translator.translate_paragraph(&para).await;

                // 更新进度
                let done = completed_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
                if let Some(ref callback) = on_progress_clone {
                    callback(done, total);
                }

                // 处理结果：失败则使用原文
                match result {
                    Ok(translated) => translated,
                    Err(e) => {
                        let err_str = e.to_string();
                        // 检查是否是内容审核错误
                        if err_str.contains("inappropriate content") ||
                           err_str.contains("data_inspection_failed") {
                            println!("[WARN] Paragraph {} skipped due to content moderation", idx);
                            failed_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                            // 返回原文作为翻译（标记为被跳过）
                            format!("[内容审核跳过] {}", para)
                        } else {
                            // 其他错误，仍然使用原文但记录错误
                            println!("[WARN] Paragraph {} translation failed: {}", idx, err_str);
                            failed_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                            para.clone()
                        }
                    }
                }
            });
            handles.push((idx, handle));
        }

        // 按原始顺序收集结果
        let mut results = vec![String::new(); paragraphs.len()];
        for (idx, handle) in handles {
            results[idx] = handle.await.map_err(|e| anyhow::anyhow!("Join error: {}", e))?;
        }

        // 打印统计
        let failed = failed_count.load(std::sync::atomic::Ordering::SeqCst);
        if failed > 0 {
            println!("[WARN] {} paragraphs failed/skipped out of {}", failed, total);
        }

        Ok(results)
    }

    /// 批量翻译段落（简化版本，无进度回调）
    pub async fn translate_paragraphs(
        &self,
        paragraphs: &[String],
        concurrency: usize,
    ) -> Result<Vec<String>> {
        Self::translate_paragraphs_with_progress(
            self.provider.clone(),
            paragraphs,
            concurrency,
            None,
        ).await
    }
}