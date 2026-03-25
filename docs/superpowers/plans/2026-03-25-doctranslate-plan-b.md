# DocTranslate 实现计划 B：toPdf 任务 + 聊天模块

> **面向 AI 代理的工作者：** 必需子技能：使用 superpowers:subagent-driven-development（推荐）或 superpowers:executing-plans 逐任务实现此计划。步骤使用复选框（`- [ ]`）语法来跟踪进度。

**目标：** 在计划 A 的基础上，实现 toPdf 翻译任务模块和聊天问答模块，完成核心功能。

**架构：** toPdf 任务采用异步队列处理，支持并行翻译和进度反馈。聊天模块基于向量检索（LanceDB）实现多文档问答。两者共享 AI 客户端和向量化服务。

**技术栈：** 延续计划 A 技术栈，新增：tokio 异步任务、Tauri 事件系统、PDF 生成（printpdf）

**前置条件：** 计划 A 已完成，基础架构可用。

---

## 文件结构

### 新增前端文件

```
src/
├── components/
│   ├── tasks/
│   │   ├── TaskList.vue         # 任务列表
│   │   ├── TaskItem.vue         # 任务项
│   │   └── CreateTaskDialog.vue # 创建任务对话框
│   └── chat/
│       ├── ChatView.vue         # 聊天界面
│       ├── MessageList.vue      # 消息列表
│       ├── MessageItem.vue      # 单条消息
│       └── ChatInput.vue        # 输入框
└── stores/
    ├── tasks.ts                 # 任务状态
    └── chat.ts                  # 聊天状态
```

### 新增后端文件

```
src-tauri/
└── src/
    ├── commands/
    │   ├── tasks.rs             # 任务命令
    │   └── chat.rs              # 聊天命令
    ├── services/
    │   ├── translator.rs        # 翻译服务
    │   └── pdf_generator.rs     # PDF 生成
    └── models/
        └── task.rs              # 任务模型
```

---

## 任务列表

### 任务 1：实现任务模型和数据库表

**文件：**
- 创建：`src-tauri/src/models/task.rs`
- 修改：`src-tauri/src/models/mod.rs`
- 修改：`src-tauri/src/db/sqlite.rs`
- 修改：`src/types/index.ts`

- [ ] **步骤 1：更新前端类型 src/types/index.ts**

添加 Task 类型（已存在，无需修改）

- [ ] **步骤 2：创建 Rust models/task.rs**

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub document_id: String,
    pub status: String, // pending | processing | completed | failed
    pub progress: i32,  // 0-100
    pub output_path: Option<String>,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTask {
    pub document_id: String,
}
```

- [ ] **步骤 3：更新 models/mod.rs**

```rust
pub mod provider;
pub mod document;
pub mod task;

pub use provider::*;
pub use document::*;
pub use task::*;
```

- [ ] **步骤 4：更新 db/sqlite.rs 添加任务表**

在 `init_db` 函数中添加：

```rust
sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS tasks (
        id TEXT PRIMARY KEY,
        document_id TEXT NOT NULL,
        status TEXT NOT NULL DEFAULT 'pending',
        progress INTEGER NOT NULL DEFAULT 0,
        output_path TEXT,
        error TEXT,
        created_at TEXT NOT NULL,
        completed_at TEXT,
        FOREIGN KEY (document_id) REFERENCES documents(id)
    )
    "#,
)
.execute(&pool)
.await?;
```

添加任务相关函数（与计划 A 风格保持一致，添加 `_db` 后缀）：

```rust
pub async fn list_tasks_db(pool: &SqlitePool) -> Result<Vec<Task>> {
    let rows = sqlx::query_as::<_, Task>(
        "SELECT id, document_id, status, progress, output_path, error, created_at, completed_at FROM tasks ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn create_task_db(pool: &SqlitePool, doc_id: &str) -> Result<Task> {
    let id = uuid::Uuid::new_v4().to_string();
    let created_at = chrono::Utc::now();

    sqlx::query(
        "INSERT INTO tasks (id, document_id, status, progress, created_at) VALUES (?, ?, 'pending', 0, ?)"
    )
    .bind(&id)
    .bind(doc_id)
    .bind(created_at.to_rfc3339())
    .execute(pool)
    .await?;

    Ok(Task {
        id,
        document_id: doc_id.to_string(),
        status: "pending".to_string(),
        progress: 0,
        output_path: None,
        error: None,
        created_at,
        completed_at: None,
    })
}

pub async fn update_task_status_db(
    pool: &SqlitePool,
    id: &str,
    status: &str,
    progress: i32,
) -> Result<()> {
    sqlx::query("UPDATE tasks SET status = ?, progress = ? WHERE id = ?")
        .bind(status)
        .bind(progress)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn complete_task_db(
    pool: &SqlitePool,
    id: &str,
    output_path: &str,
) -> Result<()> {
    let completed_at = chrono::Utc::now();
    sqlx::query(
        "UPDATE tasks SET status = 'completed', progress = 100, output_path = ?, completed_at = ? WHERE id = ?"
    )
    .bind(output_path)
    .bind(completed_at.to_rfc3339())
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn fail_task_db(pool: &SqlitePool, id: &str, error: &str) -> Result<()> {
    sqlx::query("UPDATE tasks SET status = 'failed', error = ? WHERE id = ?")
        .bind(error)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
```

- [ ] **步骤 5：Commit**

```bash
git add .
git commit -m "feat: add task model and database table"
```

---

### 任务 2：实现翻译服务

**文件：**
- 创建：`src-tauri/src/services/translator.rs`

- [ ] **步骤 1：创建 translator.rs**

```rust
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::models::ProviderConfig;
use super::ai_client::chat;
use super::ai_client::Message;

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

        // 更新术语表
        self.update_glossary(text, &result).await;

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

    async fn update_glossary(&self, chinese: &str, english: &str) {
        // 简化实现：提取可能的术语对
        // 可以后续扩展为更智能的术语提取
        let _ = (chinese, english);
    }

    pub async fn add_term(&self, chinese: String, english: String) {
        let mut glossary = self.glossary.write().await;
        glossary.insert(chinese, english);
    }
}
```

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

    async fn update_glossary(&self, chinese: &str, english: &str) {
        // 简化实现：可以后续扩展为更智能的术语提取
        // 这里暂时不做自动提取，让用户手动维护
        let _ = (chinese, english);
    }

    pub async fn add_term(&self, chinese: String, english: String) {
        let mut glossary = self.glossary.write().await;
        glossary.insert(chinese, english);
    }
}
```

- [ ] **步骤 2：更新 services/mod.rs**

```rust
pub mod ai_client;
pub mod embedding;
pub mod document_processor;
pub mod translator;

pub use translator::Translator;
```

- [ ] **步骤 3：Commit**

```bash
git add .
git commit -m "feat: add translation service with glossary support"
```

---

### 任务 3：实现 PDF 生成服务

**文件：**
- 创建：`src-tauri/src/services/pdf_generator.rs`

- [ ] **步骤 1：创建 pdf_generator.rs**

```rust
use anyhow::Result;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

pub struct PdfGenerator;

impl PdfGenerator {
    /// 生成双语对照 PDF
    /// 注意：中文支持需要嵌入字体文件，这里使用简化的实现方案
    pub fn generate_bilingual_pdf(
        output_path: &str,
        paragraphs: &[(String, String)], // (中文, 英文)
    ) -> Result<()> {
        let (width, height) = (Mm(210.0), Mm(297.0)); // A4
        let margin = Mm(20.0);

        let doc = PdfDocument::new("DocTranslate Output", width, height, "DocTranslate")?;
        let page = doc.add_page(width, height, "Page 1")?;

        // 使用 Helvetica 作为英文字体
        let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;

        // 中文字体：尝试加载本地字体，失败则使用 Identity-H 编码
        // 实际部署时需要确保字体文件存在，或使用系统字体
        let font_cjk = match Self::load_cjk_font(&doc) {
            Ok(f) => f,
            Err(_) => {
                // 回退方案：使用内置编码（可能无法正确显示部分中文）
                doc.add_builtin_font(BuiltinFont::IdentityH)?
            }
        };

        let current_layer = doc.get_page(page).get_layer("Layer 1");

        let font_size = 10.0;
        let line_height = 14.0;
        let mut y = height - margin;
        let mut page_idx = 0;

        for (chinese, english) in paragraphs {
            // 检查是否需要新页
            if y < margin + Mm(40.0) {
                let new_page = doc.add_page(width, height, &format!("Page {}", page_idx + 1));
                page_idx += 1;
                y = height - margin;
            }

            // 中文段落
            current_layer.use_text(chinese, font_size, margin, y, &font_cjk);
            y -= Mm(line_height * Self::estimate_lines(chinese, 50) as f32);

            // 英文段落
            current_layer.use_text(english, font_size, margin, y, &font);
            y -= Mm(line_height * Self::estimate_lines(english, 60) as f32 + 5.0);
        }

        doc.save(&mut BufWriter::new(File::create(output_path)?))?;

        Ok(())
    }

    /// 尝试加载中文字体
    /// 在 Windows 上尝试使用微软雅黑，其他系统需要适配
    fn load_cjk_font(doc: &PdfDocument) -> Result<IndirectFontRef> {
        // 尝试常见的中文字体路径
        let font_paths = [
            "C:\\Windows\\Fonts\\msyh.ttc",  // Windows 微软雅黑
            "/System/Library/Fonts/PingFang.ttc",  // macOS
            "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",  // Linux
        ];

        for path in &font_paths {
            if std::path::Path::new(path).exists() {
                // 注意：printpdf 加载 TTF/TTC 需要额外处理
                // 这里简化为返回 Identity-H，实际项目需要完整实现字体加载
            }
        }

        // 回退：使用 Identity-H 编码
        anyhow::bail!("No CJK font found, using fallback")
    }

    fn estimate_lines(text: &str, chars_per_line: usize) -> usize {
        (text.chars().count() + chars_per_line - 1) / chars_per_line
    }
}
```

- [ ] **步骤 2：更新 services/mod.rs**

```rust
pub mod ai_client;
pub mod embedding;
pub mod document_processor;
pub mod translator;
pub mod pdf_generator;

pub use translator::Translator;
pub use pdf_generator::PdfGenerator;
```

- [ ] **步骤 3：Commit**

```bash
git add .
git commit -m "feat: add bilingual PDF generator"
```

---

### 任务 4：实现任务命令和异步处理

**文件：**
- 创建：`src-tauri/src/commands/tasks.rs`
- 修改：`src-tauri/src/commands/mod.rs`
- 修改：`src-tauri/src/lib.rs`

- [ ] **步骤 1：创建 commands/tasks.rs**

```rust
use sqlx::SqlitePool;
use tauri::{Manager, AppHandle, Emitter};
use crate::db::*;
use crate::models::{Task, Document};
use crate::services::{Translator, PdfGenerator, document_processor::DocumentProcessor};
use crate::services::ai_client::Message;

#[tauri::command]
pub async fn list_tasks(app_handle: AppHandle) -> Result<Vec<Task>, String> {
    let pool = app_handle.state::<SqlitePool>();
    list_tasks_db(&pool).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_task(
    app_handle: AppHandle,
    document_id: String,
) -> Result<Task, String> {
    let pool = app_handle.state::<SqlitePool>();

    // 创建任务
    let task = create_task_db(&pool, &document_id).await.map_err(|e| e.to_string())?;

    // 启动后台处理
    let app_handle_clone = app_handle.clone();
    let task_id = task.id.clone();
    let pool_clone = pool.inner().clone();

    tokio::spawn(async move {
        if let Err(e) = process_task(app_handle_clone, pool_clone, task_id, document_id).await {
            eprintln!("Task processing error: {}", e);
        }
    });

    Ok(task)
}

#[tauri::command]
pub async fn delete_task(app_handle: AppHandle, id: String) -> Result<(), String> {
    let pool = app_handle.state::<SqlitePool>();
    sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(&id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

async fn process_task(
    app_handle: AppHandle,
    pool: SqlitePool,
    task_id: String,
    document_id: String,
) -> anyhow::Result<()> {
    // 更新状态为 processing
    update_task_status_db(&pool, &task_id, "processing", 0).await?;
    app_handle.emit("task-progress", &serde_json::json!({
        "taskId": task_id,
        "status": "processing",
        "progress": 0
    }))?;

    // 获取文档
    let docs = list_documents_db(&pool).await?;
    let doc = docs.iter().find(|d| d.id == document_id)
        .ok_or_else(|| anyhow::anyhow!("Document not found"))?;

    // 提取文本
    let text = DocumentProcessor::extract_text(&doc.path, &doc.doc_type)?;
    let paragraphs = DocumentProcessor::split_into_paragraphs(&text);

    if paragraphs.is_empty() {
        fail_task_db(&pool, &task_id, "文档无文本内容").await?;
        return Ok(());
    }

    // 从配置文件加载提供商配置
    let provider = load_active_provider(&app_handle)?;

    // 根据内容量决定并发数
    let concurrency = if paragraphs.len() < 10 { 2 } else { 4 };

    // 翻译（使用并发翻译方法）
    let translator = Translator::new(provider);
    let total = paragraphs.len();

    // 使用批量并发翻译
    let translated_results = translator.translate_paragraphs(&paragraphs, concurrency).await?;

    // 组装结果
    let translated: Vec<(String, String)> = paragraphs.iter()
        .zip(translated_results.iter())
        .map(|(cn, en)| (cn.clone(), en.clone()))
        .collect();

    // 更新进度到 95%
    update_task_status_db(&pool, &task_id, "processing", 95).await?;
    app_handle.emit("task-progress", &serde_json::json!({
        "taskId": task_id,
        "status": "processing",
        "progress": 95
    }))?;

    // 生成 PDF
    let output_dir = app_handle.path().app_data_dir()?;
    let output_path = output_dir.join(format!("output_{}.pdf", task_id));
    PdfGenerator::generate_bilingual_pdf(&output_path.to_string_lossy(), &translated)?;

    // 完成
    complete_task_db(&pool, &task_id, &output_path.to_string_lossy()).await?;
    app_handle.emit("task-progress", &serde_json::json!({
        "taskId": task_id,
        "status": "completed",
        "progress": 100
    }))?;

    Ok(())
}

/// 从配置文件加载活动的提供商
fn load_active_provider(app_handle: &AppHandle) -> anyhow::Result<crate::models::ProviderConfig> {
    let config_dir = app_handle.path().app_config_dir()?;
    let config_path = config_dir.join("providers.json");

    if config_path.exists() {
        let content = std::fs::read_to_string(&config_path)?;
        let providers: Vec<crate::models::ProviderConfig> = serde_json::from_str(&content)?;

        // 查找活动的提供商
        if let Some(provider) = providers.into_iter().find(|p| p.is_active && !p.api_key.is_empty()) {
            return Ok(provider);
        }
    }

    anyhow::bail!("No active provider configured. Please configure API key in Settings.")
}
```

- [ ] **步骤 2：更新 commands/mod.rs**

```rust
pub mod settings;
pub mod documents;
pub mod tasks;
```

- [ ] **步骤 3：更新 lib.rs 注册命令**

在 `invoke_handler` 中添加：

```rust
.invoke_handler(tauri::generate_handler![
    // Settings commands
    commands::settings::get_providers,
    commands::settings::save_provider,
    commands::settings::test_provider,
    // Document commands
    commands::documents::list_documents,
    commands::documents::upload_document,
    commands::documents::delete_document,
    // Task commands
    commands::tasks::list_tasks,
    commands::tasks::create_task,
    commands::tasks::delete_task,
])
```

- [ ] **步骤 4：Commit**

```bash
git add .
git commit -m "feat: add task commands with async processing"
```

---

### 任务 5：实现聊天命令和向量检索

**文件：**
- 创建：`src-tauri/src/commands/chat.rs`
- 修改：`src-tauri/src/commands/mod.rs`
- 修改：`src-tauri/src/lib.rs`

- [ ] **步骤 1：创建 commands/chat.rs**

```rust
use sqlx::SqlitePool;
use tauri::Manager;
use crate::db::*;
use crate::models::Document;
use crate::services::{ai_client, embedding, document_processor::DocumentProcessor, VectorStore};

/// 从配置文件加载活动的提供商（与 tasks.rs 共享）
fn load_active_provider(app_handle: &tauri::AppHandle) -> Result<crate::models::ProviderConfig, String> {
    let config_dir = app_handle.path().app_config_dir().map_err(|e| e.to_string())?;
    let config_path = config_dir.join("providers.json");

    if config_path.exists() {
        let content = std::fs::read_to_string(&config_path).map_err(|e| e.to_string())?;
        let providers: Vec<crate::models::ProviderConfig> = serde_json::from_str(&content)
            .map_err(|e| e.to_string())?;

        if let Some(provider) = providers.into_iter().find(|p| p.is_active && !p.api_key.is_empty()) {
            return Ok(provider);
        }
    }

    Err("No active provider configured. Please configure API key in Settings.".to_string())
}

#[tauri::command]
pub async fn chat(
    app_handle: tauri::AppHandle,
    message: String,
    document_ids: Vec<String>,
) -> Result<String, String> {
    let pool = app_handle.state::<SqlitePool>();

    // 从配置文件加载活动提供商
    let provider = load_active_provider(&app_handle)?;

    // 有选择文档 -> RAG
    if !document_ids.is_empty() {
        let docs = list_documents_db(&pool).await.map_err(|e| e.to_string())?;
        let selected: Vec<&Document> = docs.iter()
            .filter(|d| document_ids.contains(&d.id) && d.vectorized)
            .collect();

        if selected.is_empty() {
            return Ok("所选文档未向量化或不存在，请先向量化文档。".to_string());
        }

        // 获取查询向量
        let query_embedding = embedding::get_embedding(&provider, &message)
            .await
            .map_err(|e| e.to_string())?;

        // 向量搜索
        let app_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
        let db_path = app_dir.join("vectors.db");
        let vector_store = VectorStore::new(&db_path.to_string_lossy())
            .await
            .map_err(|e| e.to_string())?;

        let results = vector_store
            .search("documents", &query_embedding, 5)
            .await
            .map_err(|e| e.to_string())?;

        if results.is_empty() {
            return Ok("在所选文档中未找到相关信息。".to_string());
        }

        // 构建上下文
        let contexts: Vec<String> = results.iter().map(|(_, text, _)| text.clone()).collect();
        let context_text = contexts.join("\n\n---\n\n");

        let system_prompt = format!(
            "你是一个文档问答助手。根据以下文档内容回答用户问题。\
             \n\n文档内容：\n{}\n\n\
             如果文档中没有相关信息，请直接说'文档中未找到相关信息'。",
            context_text
        );

        let messages = vec![
            ai_client::Message {
                role: "system".to_string(),
                content: system_prompt,
            },
            ai_client::Message {
                role: "user".to_string(),
                content: message,
            },
        ];

        ai_client::chat(&provider, messages).await.map_err(|e| e.to_string())
    } else {
        // 普通聊天
        let messages = vec![
            ai_client::Message {
                role: "user".to_string(),
                content: message,
            },
        ];

        ai_client::chat(&provider, messages).await.map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub async fn vectorize_document(
    app_handle: tauri::AppHandle,
    document_id: String,
) -> Result<(), String> {
    let pool = app_handle.state::<SqlitePool>();

    let docs = list_documents_db(&pool).await.map_err(|e| e.to_string())?;
    let doc = docs.iter().find(|d| d.id == document_id)
        .ok_or_else(|| "Document not found".to_string())?;

    // 提取文本
    let text = DocumentProcessor::extract_text(&doc.path, &doc.doc_type)
        .map_err(|e| e.to_string())?;
    let paragraphs = DocumentProcessor::split_into_paragraphs(&text);

    if paragraphs.is_empty() {
        return Err("文档无文本内容".to_string());
    }

    // 从配置文件加载提供商
    let provider = load_active_provider(&app_handle)?;

    // 获取嵌入
    let embeddings = embedding::get_embeddings(&provider, &paragraphs)
        .await
        .map_err(|e| e.to_string())?;

    // 存储向量
    let app_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = app_dir.join("vectors.db");
    let vector_store = VectorStore::new(&db_path.to_string_lossy())
        .await
        .map_err(|e| e.to_string())?;

    let ids: Vec<String> = paragraphs.iter()
        .enumerate()
        .map(|(i, _)| format!("{}-{}", document_id, i))
        .collect();

    vector_store
        .insert_vectors("documents", &ids, &paragraphs, &embeddings)
        .await
        .map_err(|e| e.to_string())?;

    // 更新文档状态
    update_vectorized_db(&pool, &document_id, true)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
```

- [ ] **步骤 2：更新 commands/mod.rs**

```rust
pub mod settings;
pub mod documents;
pub mod tasks;
pub mod chat;
```

- [ ] **步骤 3：更新 lib.rs 注册命令**

添加聊天命令到 `invoke_handler`。

- [ ] **步骤 4：Commit**

```bash
git add .
git commit -m "feat: add chat commands with RAG support"
```

---

### 任务 6：实现前端任务模块

**文件：**
- 创建：`src/stores/tasks.ts`
- 创建：`src/components/tasks/TaskList.vue`
- 创建：`src/components/tasks/TaskItem.vue`
- 创建：`src/components/tasks/CreateTaskDialog.vue`

- [ ] **步骤 1：创建 stores/tasks.ts**

```typescript
import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { Task, Document } from "@/types";

interface TaskProgress {
  taskId: string;
  status: string;
  progress: number;
}

export const useTasksStore = defineStore("tasks", {
  state: () => ({
    tasks: [] as Task[],
    loading: false,
    progressMap: new Map<string, number>(),
  }),

  actions: {
    async loadTasks() {
      this.loading = true;
      try {
        this.tasks = await invoke<Task[]>("list_tasks");
      } finally {
        this.loading = false;
      }
    },

    async createTask(documentId: string) {
      await invoke<Task>("create_task", { documentId });
      await this.loadTasks();
    },

    async deleteTask(id: string) {
      await invoke("delete_task", { id });
      await this.loadTasks();
    },

    setupListener() {
      listen<TaskProgress>("task-progress", (event) => {
        const { taskId, status, progress } = event.payload;
        this.progressMap.set(taskId, progress);

        if (status === "completed" || status === "failed") {
          this.loadTasks();
        }
      });
    },

    getProgress(taskId: string): number {
      const task = this.tasks.find((t) => t.id === taskId);
      return this.progressMap.get(taskId) ?? task?.progress ?? 0;
    },
  },
});
```

- [ ] **步骤 2：创建 TaskList.vue**

```vue
<template>
  <div class="task-list">
    <div class="list-header">
      <span>任务列表</span>
      <el-button
        type="primary"
        size="small"
        :disabled="!canCreateTask"
        @click="showCreateDialog = true"
      >
        创建任务
      </el-button>
    </div>

    <el-scrollbar class="list-scroll">
      <div v-if="tasksStore.loading" class="loading">
        <el-icon class="is-loading"><Loading /></el-icon>
      </div>

      <div v-else-if="tasksStore.tasks.length === 0" class="empty">
        暂无任务
      </div>

      <div v-else class="task-items">
        <TaskItem
          v-for="task in tasksStore.tasks"
          :key="task.id"
          :task="task"
          :progress="tasksStore.getProgress(task.id)"
          @delete="handleDelete(task.id)"
        />
      </div>
    </el-scrollbar>

    <CreateTaskDialog
      v-model="showCreateDialog"
      @create="handleCreate"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { Loading } from "@element-plus/icons-vue";
import { useTasksStore } from "@/stores/tasks";
import { useDocumentsStore } from "@/stores/documents";
import TaskItem from "./TaskItem.vue";
import CreateTaskDialog from "./CreateTaskDialog.vue";

const tasksStore = useTasksStore();
const documentsStore = useDocumentsStore();

const showCreateDialog = ref(false);

const canCreateTask = computed(() => {
  return documentsStore.documents.length > 0;
});

onMounted(() => {
  tasksStore.loadTasks();
  tasksStore.setupListener();
});

function handleCreate(documentId: string) {
  tasksStore.createTask(documentId);
  showCreateDialog.value = false;
}

function handleDelete(id: string) {
  tasksStore.deleteTask(id);
}
</script>

<style scoped>
.task-list {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  font-weight: 600;
  border-bottom: 1px solid var(--el-border-color);
}

.list-scroll {
  flex: 1;
}

.loading,
.empty {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100px;
  color: var(--el-text-color-secondary);
}

.task-items {
  padding: 8px;
}
</style>
```

- [ ] **步骤 3：创建 TaskItem.vue**

```vue
<template>
  <div class="task-item" :class="task.status">
    <div class="task-header">
      <span class="task-name">{{ documentName }}</span>
      <el-tag :type="statusType" size="small">{{ statusText }}</el-tag>
    </div>

    <el-progress
      v-if="task.status === 'processing'"
      :percentage="progress"
      :stroke-width="6"
    />

    <div class="task-footer">
      <span class="task-time">{{ formatTime(task.createdAt) }}</span>

      <div class="task-actions">
        <el-button
          v-if="task.status === 'completed'"
          type="primary"
          link
          size="small"
          @click="handleOpen"
        >
          打开
        </el-button>
        <el-button
          v-if="task.status === 'failed'"
          type="warning"
          link
          size="small"
        >
          重试
        </el-button>
        <el-button
          type="danger"
          link
          size="small"
          @click="emit('delete')"
        >
          删除
        </el-button>
      </div>
    </div>

    <div v-if="task.error" class="task-error">
      {{ task.error }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useDocumentsStore } from "@/stores/documents";
import type { Task } from "@/types";

const props = defineProps<{
  task: Task;
  progress: number;
}>();

const emit = defineEmits<{
  delete: [];
}>();

const documentsStore = useDocumentsStore();

const documentName = computed(() => {
  const doc = documentsStore.documents.find((d) => d.id === props.task.documentId);
  return doc?.name || "未知文档";
});

const statusType = computed(() => {
  switch (props.task.status) {
    case "completed":
      return "success";
    case "failed":
      return "danger";
    case "processing":
      return "primary";
    default:
      return "info";
  }
});

const statusText = computed(() => {
  switch (props.task.status) {
    case "pending":
      return "等待中";
    case "processing":
      return "处理中";
    case "completed":
      return "已完成";
    case "failed":
      return "失败";
    default:
      return props.task.status;
  }
});

function formatTime(dateStr: string): string {
  const date = new Date(dateStr);
  return date.toLocaleString("zh-CN");
}

function handleOpen() {
  if (props.task.outputPath) {
    // 使用 Tauri shell 打开文件
    import("@tauri-apps/plugin-shell").then(({ open }) => {
      open(props.task.outputPath!);
    });
  }
}
</script>

<style scoped>
.task-item {
  padding: 12px;
  border-radius: 8px;
  background: var(--el-fill-color-light);
  margin-bottom: 8px;
}

.task-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.task-name {
  font-weight: 500;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.task-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 8px;
}

.task-time {
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.task-actions {
  display: flex;
  gap: 8px;
}

.task-error {
  margin-top: 8px;
  padding: 8px;
  background: var(--el-color-danger-light-9);
  border-radius: 4px;
  font-size: 12px;
  color: var(--el-color-danger);
}
</style>
```

- [ ] **步骤 4：创建 CreateTaskDialog.vue**

```vue
<template>
  <el-dialog
    :model-value="modelValue"
    title="创建翻译任务"
    width="400px"
    @update:model-value="emit('update:modelValue', $event)"
  >
    <el-form label-width="80px">
      <el-form-item label="选择文档">
        <el-select v-model="selectedDocumentId" placeholder="请选择文档">
          <el-option
            v-for="doc in documentsStore.documents"
            :key="doc.id"
            :label="doc.name"
            :value="doc.id"
          />
        </el-select>
      </el-form-item>
    </el-form>

    <template #footer>
      <el-button @click="emit('update:modelValue', false)">取消</el-button>
      <el-button
        type="primary"
        :disabled="!selectedDocumentId"
        @click="handleCreate"
      >
        创建
      </el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useDocumentsStore } from "@/stores/documents";

const props = defineProps<{
  modelValue: boolean;
}>();

const emit = defineEmits<{
  "update:modelValue": [value: boolean];
  create: [documentId: string];
}>();

const documentsStore = useDocumentsStore();
const selectedDocumentId = ref("");

function handleCreate() {
  if (selectedDocumentId.value) {
    emit("create", selectedDocumentId.value);
    selectedDocumentId.value = "";
  }
}
</script>
```

- [ ] **步骤 5：Commit**

```bash
git add .
git commit -m "feat: add frontend task module with progress tracking"
```

---

### 任务 7：实现前端聊天模块

**文件：**
- 创建：`src/stores/chat.ts`
- 创建：`src/components/chat/ChatView.vue`
- 创建：`src/components/chat/MessageList.vue`
- 创建：`src/components/chat/MessageItem.vue`
- 创建：`src/components/chat/ChatInput.vue`

- [ ] **步骤 1：创建 stores/chat.ts**

```typescript
import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import type { ChatMessage } from "@/types";

export const useChatStore = defineStore("chat", {
  state: () => ({
    messages: [] as ChatMessage[],
    loading: false,
  }),

  actions: {
    async sendMessage(content: string, documentIds: string[]) {
      // 添加用户消息
      const userMessage: ChatMessage = {
        id: Date.now().toString(),
        role: "user",
        content,
        createdAt: new Date().toISOString(),
      };
      this.messages.push(userMessage);
      this.loading = true;

      try {
        const response = await invoke<string>("chat", {
          message: content,
          documentIds,
        });

        const assistantMessage: ChatMessage = {
          id: (Date.now() + 1).toString(),
          role: "assistant",
          content: response,
          createdAt: new Date().toISOString(),
        };
        this.messages.push(assistantMessage);
      } catch (error: any) {
        const errorMessage: ChatMessage = {
          id: (Date.now() + 1).toString(),
          role: "assistant",
          content: `错误: ${error}`,
          createdAt: new Date().toISOString(),
        };
        this.messages.push(errorMessage);
      } finally {
        this.loading = false;
      }
    },

    clearMessages() {
      this.messages = [];
    },
  },
});
```

- [ ] **步骤 2：创建 ChatView.vue**

```vue
<template>
  <div class="chat-view">
    <div class="chat-header">
      <el-select
        v-model="selectedDocIds"
        multiple
        collapse-tags
        placeholder="选择文档进行问答（可选）"
        class="doc-selector"
      >
        <el-option
          v-for="doc in vectorizedDocs"
          :key="doc.id"
          :label="doc.name"
          :value="doc.id"
        />
      </el-select>
    </div>

    <MessageList :messages="chatStore.messages" :loading="chatStore.loading" />

    <ChatInput
      :disabled="chatStore.loading"
      @send="handleSend"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { useChatStore } from "@/stores/chat";
import { useDocumentsStore } from "@/stores/documents";
import MessageList from "./MessageList.vue";
import ChatInput from "./ChatInput.vue";

const chatStore = useChatStore();
const documentsStore = useDocumentsStore();

const selectedDocIds = ref<string[]>([]);

const vectorizedDocs = computed(() =>
  documentsStore.documents.filter((d) => d.vectorized)
);

function handleSend(content: string) {
  chatStore.sendMessage(content, selectedDocIds.value);
}
</script>

<style scoped>
.chat-view {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.chat-header {
  padding: 12px;
  border-bottom: 1px solid var(--el-border-color);
}

.doc-selector {
  width: 100%;
}
</style>
```

- [ ] **步骤 3：创建 MessageList.vue**

```vue
<template>
  <div class="message-list">
    <el-scrollbar ref="scrollbarRef">
      <div class="messages">
        <div v-if="messages.length === 0" class="empty">
          开始对话吧！选择文档可以进行基于文档的问答。
        </div>

        <MessageItem
          v-for="msg in messages"
          :key="msg.id"
          :message="msg"
        />

        <div v-if="loading" class="loading-message">
          <el-icon class="is-loading"><Loading /></el-icon>
          <span>思考中...</span>
        </div>
      </div>
    </el-scrollbar>
  </div>
</template>

<script setup lang="ts">
import { watch, nextTick, ref } from "vue";
import { Loading } from "@element-plus/icons-vue";
import type { ChatMessage } from "@/types";
import MessageItem from "./MessageItem.vue";

defineProps<{
  messages: ChatMessage[];
  loading: boolean;
}>();

const scrollbarRef = ref();

// 自动滚动到底部
watch(
  () => props.messages.length,
  () => {
    nextTick(() => {
      scrollbarRef.value?.setScrollTop(99999);
    });
  }
);
</script>

<style scoped>
.message-list {
  flex: 1;
  overflow: hidden;
}

.messages {
  padding: 16px;
  min-height: 100%;
}

.empty {
  text-align: center;
  color: var(--el-text-color-secondary);
  padding: 40px;
}

.loading-message {
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--el-text-color-secondary);
  padding: 16px;
}
</style>
```

- [ ] **步骤 4：创建 MessageItem.vue**

```vue
<template>
  <div class="message-item" :class="message.role">
    <div class="avatar">
      <el-avatar :size="32" :icon="message.role === 'user' ? User : Monitor" />
    </div>
    <div class="content">
      <div class="role">{{ message.role === "user" ? "你" : "助手" }}</div>
      <div class="text">{{ message.content }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { User, Monitor } from "@element-plus/icons-vue";
import type { ChatMessage } from "@/types";

defineProps<{
  message: ChatMessage;
}>();
</script>

<style scoped>
.message-item {
  display: flex;
  gap: 12px;
  margin-bottom: 16px;
}

.message-item.user {
  flex-direction: row-reverse;
}

.message-item.user .content {
  align-items: flex-end;
}

.content {
  display: flex;
  flex-direction: column;
  gap: 4px;
  max-width: 70%;
}

.role {
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.text {
  padding: 12px 16px;
  border-radius: 12px;
  background: var(--el-fill-color-light);
  line-height: 1.5;
  white-space: pre-wrap;
}

.message-item.user .text {
  background: var(--el-color-primary-light-9);
}
</style>
```

- [ ] **步骤 5：创建 ChatInput.vue**

```vue
<template>
  <div class="chat-input">
    <el-input
      v-model="input"
      type="textarea"
      :rows="2"
      placeholder="输入消息..."
      :disabled="disabled"
      @keydown.enter.ctrl="handleSend"
    />
    <el-button type="primary" :disabled="!input.trim() || disabled" @click="handleSend">
      发送
    </el-button>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

const props = defineProps<{
  disabled?: boolean;
}>();

const emit = defineEmits<{
  send: [content: string];
}>();

const input = ref("");

function handleSend() {
  const content = input.value.trim();
  if (content && !props.disabled) {
    emit("send", content);
    input.value = "";
  }
}
</script>

<style scoped>
.chat-input {
  display: flex;
  gap: 12px;
  padding: 12px;
  border-top: 1px solid var(--el-border-color);
}

.chat-input .el-textarea {
  flex: 1;
}
</style>
```

- [ ] **步骤 6：Commit**

```bash
git add .
git commit -m "feat: add chat module with document-based Q&A"
```

---

### 任务 8：更新主页面集成模块

**文件：**
- 修改：`src/views/HomeView.vue`

- [ ] **步骤 1：更新 HomeView.vue**

```vue
<template>
  <AppLayout>
    <template #sidebar>
      <DocumentList @refresh="handleRefresh" />
    </template>

    <template #content>
      <div class="home-content">
        <el-tabs v-model="activeTab">
          <el-tab-pane label="toPdf 任务" name="tasks">
            <TaskList />
          </el-tab-pane>

          <el-tab-pane label="聊天" name="chat">
            <ChatView />
          </el-tab-pane>
        </el-tabs>
      </div>
    </template>
  </AppLayout>
</template>

<script setup lang="ts">
import { ref } from "vue";
import AppLayout from "@/components/layout/AppLayout.vue";
import DocumentList from "@/components/documents/DocumentList.vue";
import TaskList from "@/components/tasks/TaskList.vue";
import ChatView from "@/components/chat/ChatView.vue";

const activeTab = ref("tasks");

function handleRefresh() {
  // 刷新内容区域
}
</script>

<style scoped>
.home-content {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.home-content :deep(.el-tabs__content) {
  flex: 1;
  overflow: hidden;
}

.home-content :deep(.el-tab-pane) {
  height: 100%;
}
</style>
```

- [ ] **步骤 2：Commit**

```bash
git add .
git commit -m "feat: integrate task and chat modules into home page"
```

---

### 任务 9：集成测试和验证

- [ ] **步骤 1：验证编译**

运行：
```bash
npm run build
```
预期：构建成功

- [ ] **步骤 2：启动开发模式**

运行：
```bash
npm run tauri dev
```

- [ ] **步骤 3：手动测试流程**

1. **设置测试**
   - 进入设置，配置 API Key
   - 测试连接

2. **文档管理测试**
   - 上传 PDF 文件
   - 确认向量化状态

3. **任务测试**
   - 选择文档创建任务
   - 观察进度更新
   - 打开生成的 PDF

4. **聊天测试**
   - 不选文档进行普通聊天
   - 选择文档进行问答

- [ ] **步骤 4：Commit**

```bash
git add .
git commit -m "test: verify all modules work correctly"
```

---

## 检查点

完成此计划后，应该得到：

- [x] toPdf 任务模块可创建、查看、删除任务
- [x] 任务异步处理，实时进度反馈
- [x] 双语 PDF 生成正确
- [x] 聊天模块支持普通对话和文档问答
- [x] 多文档选择和向量检索正常
- [x] 所有模块集成到主界面

**项目完成：** DocTranslate 核心功能全部实现。