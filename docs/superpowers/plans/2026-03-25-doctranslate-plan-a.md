# DocTranslate 实现计划 A：基础架构 + 设置 + 文档管理

> **面向 AI 代理的工作者：** 必需子技能：使用 superpowers:subagent-driven-development（推荐）或 superpowers:executing-plans 逐任务实现此计划。步骤使用复选框（`- [ ]`）语法来跟踪进度。

**目标：** 搭建 Tauri 2.0 + Vue 3 + Element Plus 项目骨架，实现设置模块和文档管理模块，产出一个可独立运行的第一个版本。

**架构：** 前端 Vue 3 负责全部 UI，通过 Tauri IPC 调用 Rust 后端。Rust 后端处理配置存储、文件操作、AI API 调用、向量化。数据存储使用 SQLite（元数据）+ LanceDB（向量）+ Tauri 配置文件（设置）。

**技术栈：** Tauri 2.0、Vue 3、TypeScript、Element Plus、Rust、SQLite、LanceDB、reqwest（HTTP 客户端）

---

## 文件结构

### 前端（Vue 3 + TypeScript）

```
src/
├── App.vue                      # 主应用入口
├── main.ts                      # Vue 入口
├── style.css                    # 全局样式
├── router/
│   └── index.ts                 # 路由配置
├── stores/
│   ├── index.ts                 # Pinia 入口
│   ├── settings.ts              # 设置状态
│   └── documents.ts             # 文档状态
├── views/
│   ├── SettingsView.vue         # 设置页面
│   └── HomeView.vue             # 主页
├── components/
│   ├── layout/
│   │   └── AppLayout.vue        # 分栏布局
│   ├── settings/
│   │   └── ProviderConfig.vue   # 提供商配置表单
│   └── documents/
│       ├── DocumentList.vue     # 文档列表
│       └── UploadButton.vue     # 上传按钮
└── types/
    └── index.ts                 # TypeScript 类型
```

### 后端（Rust）

```
src-tauri/
├── Cargo.toml
├── tauri.conf.json
├── capabilities/
│   └── default.json
└── src/
    ├── main.rs                  # 入口
    ├── lib.rs                   # 库入口
    ├── commands/
    │   ├── mod.rs
    │   ├── settings.rs          # 设置命令
    │   └── documents.rs         # 文档命令
    ├── services/
    │   ├── mod.rs
    │   ├── ai_client.rs         # AI API 客户端
    │   ├── embedding.rs         # 嵌入服务
    │   └── document_processor.rs # 文档解析
    ├── models/
    │   ├── mod.rs
    │   ├── provider.rs          # 提供商模型
    │   └── document.rs          # 文档模型
    ├── db/
    │   ├── mod.rs
    │   └── sqlite.rs            # SQLite 操作
    └── vector/
        ├── mod.rs
        └── lancedb_store.rs     # LanceDB 操作
```

---

## 任务列表

### 任务 1：初始化 Tauri 2.0 项目

**文件：**
- 创建：`src-tauri/Cargo.toml`
- 创建：`src-tauri/tauri.conf.json`
- 创建：`src-tauri/capabilities/default.json`
- 创建：`src-tauri/src/main.rs`
- 创建：`src-tauri/src/lib.rs`
- 创建：`package.json`
- 创建：`vite.config.ts`
- 创建：`tsconfig.json`
- 创建：`index.html`
- 创建：`src/main.ts`
- 创建：`src/App.vue`
- 创建：`src/style.css`

- [ ] **步骤 1：创建前端项目目录和基础配置**

运行：
```bash
mkdir -p src router stores views components/settings components/documents components/layout types
```

- [ ] **步骤 2：创建 package.json**

```json
{
  "name": "doctranslate",
  "version": "0.1.0",
  "private": true,
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vue-tsc --noEmit && vite build",
    "preview": "vite preview",
    "tauri": "tauri"
  },
  "dependencies": {
    "vue": "^3.4.0",
    "vue-router": "^4.3.0",
    "pinia": "^2.1.0",
    "element-plus": "^2.5.0",
    "@element-plus/icons-vue": "^2.3.0",
    "@tauri-apps/plugin-dialog": "^2.0.0"
  },
  "devDependencies": {
    "@vitejs/plugin-vue": "^5.0.0",
    "@tauri-apps/api": "^2.0.0",
    "@tauri-apps/cli": "^2.0.0",
    "typescript": "^5.3.0",
    "vite": "^5.0.0",
    "vue-tsc": "^2.0.0"
  }
}
```

- [ ] **步骤 3：创建 vite.config.ts**

```typescript
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

export default defineConfig({
  plugins: [vue()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
});
```

- [ ] **步骤 4：创建 tsconfig.json**

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "module": "ESNext",
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "preserve",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true,
    "paths": {
      "@/*": ["./src/*"]
    }
  },
  "include": ["src/**/*.ts", "src/**/*.tsx", "src/**/*.vue"],
  "references": [{ "path": "./tsconfig.node.json" }]
}
```

- [ ] **步骤 5：创建 tsconfig.node.json**

```json
{
  "compilerOptions": {
    "composite": true,
    "skipLibCheck": true,
    "module": "ESNext",
    "moduleResolution": "bundler",
    "allowSyntheticDefaultImports": true,
    "strict": true
  },
  "include": ["vite.config.ts"]
}
```

- [ ] **步骤 6：创建 index.html**

```html
<!DOCTYPE html>
<html lang="zh-CN">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>DocTranslate</title>
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/src/main.ts"></script>
  </body>
</html>
```

- [ ] **步骤 7：创建 src/main.ts**

```typescript
import { createApp } from "vue";
import { createPinia } from "pinia";
import ElementPlus from "element-plus";
import "element-plus/dist/index.css";
import * as ElementPlusIconsVue from "@element-plus/icons-vue";
import router from "./router";
import App from "./App.vue";
import "./style.css";

const app = createApp(App);

// 注册所有图标
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component);
}

app.use(createPinia());
app.use(router);
app.use(ElementPlus);
app.mount("#app");
```

- [ ] **步骤 8：创建 src/App.vue**

```vue
<template>
  <router-view />
</template>

<script setup lang="ts">
</script>
```

- [ ] **步骤 9：创建 src/style.css**

```css
:root {
  font-family: Inter, system-ui, Avenir, Helvetica, Arial, sans-serif;
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html, body, #app {
  height: 100%;
  width: 100%;
}
```

- [ ] **步骤 10：创建 Tauri 后端目录**

运行：
```bash
mkdir -p src-tauri/src/commands src-tauri/src/services src-tauri/src/models src-tauri/src/db src-tauri/src/vector src-tauri/capabilities
```

- [ ] **步骤 11：创建 src-tauri/Cargo.toml**

```toml
[package]
name = "doctranslate"
version = "0.1.0"
edition = "2021"

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-shell = "2"
tauri-plugin-dialog = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
sqlx = { version = "0.7", features = ["runtime-tokio", "sqlite"] }
uuid = { version = "1", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1"
thiserror = "1"
lancedb = "0.4"
arrow = "53"
pdf-extract = "0.7"
docx-rs = "0.8"
printpdf = "0.7"
lopdf = "0.34"
zip = "2"

[features]
default = ["custom-protocol"]
custom-protocol = ["tauri/custom-protocol"]
```

- [ ] **步骤 12：创建 src-tauri/tauri.conf.json**

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "DocTranslate",
  "version": "0.1.0",
  "identifier": "com.doctranslate.app",
  "build": {
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../dist"
  },
  "app": {
    "withGlobalTauri": true,
    "windows": [
      {
        "title": "DocTranslate",
        "width": 1200,
        "height": 800,
        "resizable": true,
        "fullscreen": false
      }
    ]
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

- [ ] **步骤 13：创建 src-tauri/capabilities/default.json**

```json
{
  "$schema": "https://schema.tauri.app/config/2.0.0/capabilities",
  "identifier": "default",
  "description": "Default capabilities for the app",
  "permissions": [
    "core:default",
    "shell:allow-open",
    "dialog:default"
  ]
}
```

- [ ] **步骤 14：创建 src-tauri/build.rs**

```rust
fn main() {
    tauri_build::build()
}
```

- [ ] **步骤 15：创建 src-tauri/src/main.rs**

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    doctranslate_lib::run()
}
```

- [ ] **步骤 16：创建 src-tauri/src/lib.rs**

```rust
mod commands;
mod db;
mod models;
mod services;
mod vector;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let app_handle = app.handle();
            tauri::async_runtime::block_on(async {
                if let Err(e) = db::init_db(&app_handle).await {
                    eprintln!("Failed to initialize database: {}", e);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Settings commands
            commands::settings::get_providers,
            commands::settings::save_provider,
            commands::settings::test_provider,
            // Document commands
            commands::documents::list_documents,
            commands::documents::upload_document,
            commands::documents::delete_document,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **步骤 17：安装依赖并验证项目结构**

运行：
```bash
npm install
```

- [ ] **步骤 18：Commit**

```bash
git add .
git commit -m "chore: initialize Tauri 2.0 + Vue 3 project structure"
```

---

### 任务 2：实现数据模型和数据库

**文件：**
- 创建：`src/types/index.ts`
- 创建：`src-tauri/src/models/mod.rs`
- 创建：`src-tauri/src/models/provider.rs`
- 创建：`src-tauri/src/models/document.rs`
- 创建：`src-tauri/src/db/mod.rs`
- 创建：`src-tauri/src/db/sqlite.rs`

- [ ] **步骤 1：创建前端类型定义 src/types/index.ts**

```typescript
// 模型提供商配置
export interface ProviderConfig {
  id: string; // deepseek | moonshot | zhipu | bailian
  name: string;
  apiKey: string;
  baseUrl: string;
  model: string;
  embeddingModel: string;
  isActive: boolean;
}

// 文档
export interface Document {
  id: string;
  name: string;
  type: "word" | "pdf";
  size: number;
  path: string;
  createdAt: string;
  vectorized: boolean;
}

// 任务
export interface Task {
  id: string;
  documentId: string;
  status: "pending" | "processing" | "completed" | "failed";
  progress: number;
  outputPath?: string;
  error?: string;
  createdAt: string;
  completedAt?: string;
}

// 聊天消息
export interface ChatMessage {
  id: string;
  role: "user" | "assistant";
  content: string;
  createdAt: string;
}

// 提供商预设
export const PROVIDER_PRESETS: Record<string, Partial<ProviderConfig>> = {
  deepseek: {
    id: "deepseek",
    name: "DeepSeek",
    baseUrl: "https://api.deepseek.com",
    model: "deepseek-chat",
    embeddingModel: "deepseek-embedding",
  },
  moonshot: {
    id: "moonshot",
    name: "Moonshot",
    baseUrl: "https://api.moonshot.cn",
    model: "moonshot-v1-8k",
    embeddingModel: "moonshot-embedding-v1",
  },
  zhipu: {
    id: "zhipu",
    name: "智谱",
    baseUrl: "https://open.bigmodel.cn/api/paas/v4",
    model: "glm-4",
    embeddingModel: "embedding-3",
  },
  bailian: {
    id: "bailian",
    name: "百炼",
    baseUrl: "https://dashscope.aliyuncs.com/api/v1",
    model: "qwen-turbo",
    embeddingModel: "text-embedding-v2",
  },
};
```

- [ ] **步骤 2：创建 Rust models/mod.rs**

```rust
pub mod provider;
pub mod document;

pub use provider::*;
pub use document::*;
```

- [ ] **步骤 3：创建 Rust models/provider.rs**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub id: String,
    pub name: String,
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub embedding_model: String,
    pub is_active: bool,
}

impl ProviderConfig {
    pub fn presets() -> Vec<ProviderConfig> {
        vec![
            ProviderConfig {
                id: "deepseek".to_string(),
                name: "DeepSeek".to_string(),
                api_key: String::new(),
                base_url: "https://api.deepseek.com".to_string(),
                model: "deepseek-chat".to_string(),
                embedding_model: "deepseek-embedding".to_string(),
                is_active: false,
            },
            ProviderConfig {
                id: "moonshot".to_string(),
                name: "Moonshot".to_string(),
                api_key: String::new(),
                base_url: "https://api.moonshot.cn".to_string(),
                model: "moonshot-v1-8k".to_string(),
                embedding_model: "moonshot-embedding-v1".to_string(),
                is_active: false,
            },
            ProviderConfig {
                id: "zhipu".to_string(),
                name: "智谱".to_string(),
                api_key: String::new(),
                base_url: "https://open.bigmodel.cn/api/paas/v4".to_string(),
                model: "glm-4".to_string(),
                embedding_model: "embedding-3".to_string(),
                is_active: false,
            },
            ProviderConfig {
                id: "bailian".to_string(),
                name: "百炼".to_string(),
                api_key: String::new(),
                base_url: "https://dashscope.aliyuncs.com/api/v1".to_string(),
                model: "qwen-turbo".to_string(),
                embedding_model: "text-embedding-v2".to_string(),
                is_active: false,
            },
        ]
    }
}
```

- [ ] **步骤 4：创建 Rust models/document.rs**

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Document {
    pub id: String,
    pub name: String,
    pub doc_type: String, // "word" | "pdf"
    pub size: i64,
    pub path: String,
    pub created_at: DateTime<Utc>,
    pub vectorized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDocument {
    pub name: String,
    pub doc_type: String,
    pub size: i64,
    pub path: String,
}
```

- [ ] **步骤 5：创建 Rust db/mod.rs**

```rust
mod sqlite;

pub use sqlite::*;
```

- [ ] **步骤 6：创建 Rust db/sqlite.rs**

```rust
use anyhow::Result;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use tauri::Manager;
use crate::models::{Document, CreateDocument};

pub async fn init_db(app_handle: &tauri::AppHandle) -> Result<()> {
    let app_dir = app_handle.path().app_data_dir()?;
    std::fs::create_dir_all(&app_dir)?;

    let db_path = app_dir.join("doctranslate.db");
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // 创建表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS documents (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            doc_type TEXT NOT NULL,
            size INTEGER NOT NULL,
            path TEXT NOT NULL,
            created_at TEXT NOT NULL,
            vectorized INTEGER NOT NULL DEFAULT 0
        )
        "#,
    )
    .execute(&pool)
    .await?;

    // 存储 pool 到 app state
    app_handle.manage(pool);

    Ok(())
}

pub async fn list_documents(pool: &SqlitePool) -> Result<Vec<Document>> {
    let rows = sqlx::query_as::<_, Document>(
        "SELECT id, name, doc_type as doc_type, size, path, created_at, vectorized FROM documents ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn create_document(pool: &SqlitePool, doc: CreateDocument) -> Result<Document> {
    let id = uuid::Uuid::new_v4().to_string();
    let created_at = chrono::Utc::now();

    sqlx::query(
        "INSERT INTO documents (id, name, doc_type, size, path, created_at, vectorized) VALUES (?, ?, ?, ?, ?, ?, 0)"
    )
    .bind(&id)
    .bind(&doc.name)
    .bind(&doc.doc_type)
    .bind(doc.size)
    .bind(&doc.path)
    .bind(created_at.to_rfc3339())
    .execute(pool)
    .await?;

    Ok(Document {
        id,
        name: doc.name,
        doc_type: doc.doc_type,
        size: doc.size,
        path: doc.path,
        created_at,
        vectorized: false,
    })
}

pub async fn delete_document(pool: &SqlitePool, id: &str) -> Result<()> {
    sqlx::query("DELETE FROM documents WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn update_vectorized(pool: &SqlitePool, id: &str, vectorized: bool) -> Result<()> {
    sqlx::query("UPDATE documents SET vectorized = ? WHERE id = ?")
        .bind(vectorized)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
```

- [ ] **步骤 7：Commit**

```bash
git add .
git commit -m "feat: add data models and SQLite database layer"
```

---

### 任务 3：实现设置命令和 AI 客户端

**文件：**
- 创建：`src-tauri/src/commands/mod.rs`
- 创建：`src-tauri/src/commands/settings.rs`
- 创建：`src-tauri/src/services/mod.rs`
- 创建：`src-tauri/src/services/ai_client.rs`

- [ ] **步骤 1：创建 commands/mod.rs**

```rust
pub mod settings;
pub mod documents;
```

- [ ] **步骤 2：创建 commands/settings.rs**

```rust
use tauri::Manager;
use crate::models::ProviderConfig;
use crate::services::ai_client::test_connection;

#[tauri::command]
pub async fn get_providers() -> Vec<ProviderConfig> {
    ProviderConfig::presets()
}

#[tauri::command]
pub async fn save_provider(
    app_handle: tauri::AppHandle,
    provider: ProviderConfig,
) -> Result<(), String> {
    let config_dir = app_handle.path().app_config_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;

    let config_path = config_dir.join("providers.json");
    let mut providers: Vec<ProviderConfig> = if config_path.exists() {
        let content = std::fs::read_to_string(&config_path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        ProviderConfig::presets()
    };

    // 更新或添加 provider
    if let Some(existing) = providers.iter_mut().find(|p| p.id == provider.id) {
        *existing = provider;
    } else {
        providers.push(provider);
    }

    let content = serde_json::to_string_pretty(&providers).map_err(|e| e.to_string())?;
    std::fs::write(&config_path, content).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn test_provider(provider: ProviderConfig) -> Result<String, String> {
    test_connection(&provider).await
}
```

- [ ] **步骤 3：创建 services/mod.rs**

```rust
pub mod ai_client;
pub mod embedding;
pub mod document_processor;
```

- [ ] **步骤 4：创建 services/ai_client.rs**

```rust
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::models::ProviderConfig;

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

pub async fn test_connection(provider: &ProviderConfig) -> Result<String> {
    let client = Client::new();

    let url = format!("{}/chat/completions", provider.base_url);

    let request = ChatRequest {
        model: provider.model.clone(),
        messages: vec![Message {
            role: "user".to_string(),
            content: "Hello".to_string(),
        }],
        max_tokens: 10,
    };

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", provider.api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow!("API error: {}", error_text));
    }

    let chat_response: ChatResponse = response.json().await?;

    chat_response
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .ok_or_else(|| anyhow!("No response from API"))
}

pub async fn chat(
    provider: &ProviderConfig,
    messages: Vec<Message>,
) -> Result<String> {
    let client = Client::new();
    let url = format!("{}/chat/completions", provider.base_url);

    let request = ChatRequest {
        model: provider.model.clone(),
        messages,
        max_tokens: 4096,
    };

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", provider.api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow!("API error: {}", error_text));
    }

    let chat_response: ChatResponse = response.json().await?;

    chat_response
        .choices
        .first()
        .map(|c| c.message.content.clone())
        .ok_or_else(|| anyhow!("No response from API"))
}
```

- [ ] **步骤 5：Commit**

```bash
git add .
git commit -m "feat: add settings commands and AI client"
```

---

### 任务 4：实现文档命令和向量化服务

**文件：**
- 创建：`src-tauri/src/commands/documents.rs`
- 创建：`src-tauri/src/services/embedding.rs`
- 创建：`src-tauri/src/services/document_processor.rs`
- 创建：`src-tauri/src/vector/mod.rs`
- 创建：`src-tauri/src/vector/lancedb_store.rs`

- [ ] **步骤 1：创建 commands/documents.rs**

```rust
use sqlx::SqlitePool;
use tauri::Manager;
use crate::db::{list_documents as db_list_documents, create_document, delete_document as db_delete_document};
use crate::models::{Document, CreateDocument};

#[tauri::command]
pub async fn list_documents(app_handle: tauri::AppHandle) -> Result<Vec<Document>, String> {
    let pool = app_handle.state::<SqlitePool>();
    db_list_documents(&pool).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn upload_document(
    app_handle: tauri::AppHandle,
    file_path: String,
) -> Result<Document, String> {
    let pool = app_handle.state::<SqlitePool>();

    // 获取文件信息
    let path = std::path::Path::new(&file_path);
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let doc_type = match extension.as_str() {
        "doc" | "docx" => "word",
        "pdf" => "pdf",
        _ => return Err("Unsupported file type".to_string()),
    };

    let metadata = std::fs::metadata(&file_path).map_err(|e| e.to_string())?;
    let size = metadata.len() as i64;

    // 复制文件到应用数据目录
    let app_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    let docs_dir = app_dir.join("documents");
    std::fs::create_dir_all(&docs_dir).map_err(|e| e.to_string())?;

    let dest_path = docs_dir.join(&name);
    std::fs::copy(&file_path, &dest_path).map_err(|e| e.to_string())?;

    let doc = CreateDocument {
        name,
        doc_type: doc_type.to_string(),
        size,
        path: dest_path.to_string_lossy().to_string(),
    };

    create_document(&pool, doc).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_document(
    app_handle: tauri::AppHandle,
    id: String,
) -> Result<(), String> {
    let pool = app_handle.state::<SqlitePool>();

    // 获取文档路径并删除文件
    let docs = db_list_documents(&pool).await.map_err(|e| e.to_string())?;
    if let Some(doc) = docs.iter().find(|d| d.id == id) {
        if std::path::Path::new(&doc.path).exists() {
            std::fs::remove_file(&doc.path).map_err(|e| e.to_string())?;
        }
    }

    db_delete_document(&pool, &id).await.map_err(|e| e.to_string())
}
```

- [ ] **步骤 2：创建 services/embedding.rs**

```rust
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::models::ProviderConfig;

#[derive(Debug, Serialize)]
struct EmbeddingRequest {
    model: String,
    input: String,
}

#[derive(Debug, Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
}

#[derive(Debug, Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
}

pub async fn get_embedding(provider: &ProviderConfig, text: &str) -> Result<Vec<f32>> {
    let client = Client::new();
    let url = format!("{}/embeddings", provider.base_url);

    let request = EmbeddingRequest {
        model: provider.embedding_model.clone(),
        input: text.to_string(),
    };

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", provider.api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow!("Embedding API error: {}", error_text));
    }

    let embedding_response: EmbeddingResponse = response.json().await?;

    embedding_response
        .data
        .first()
        .map(|d| d.embedding.clone())
        .ok_or_else(|| anyhow!("No embedding returned"))
}

pub async fn get_embeddings(provider: &ProviderConfig, texts: &[String]) -> Result<Vec<Vec<f32>>> {
    let mut results = Vec::new();
    for text in texts {
        let embedding = get_embedding(provider, text).await?;
        results.push(embedding);
    }
    Ok(results)
}
```

- [ ] **步骤 3：创建 services/document_processor.rs**

```rust
use anyhow::Result;
use std::io::Read;

pub struct DocumentProcessor;

impl DocumentProcessor {
    pub fn extract_text(file_path: &str, doc_type: &str) -> Result<String> {
        match doc_type {
            "pdf" => Self::extract_pdf_text(file_path),
            "word" => Self::extract_word_text(file_path),
            _ => Err(anyhow::anyhow!("Unsupported document type")),
        }
    }

    fn extract_pdf_text(file_path: &str) -> Result<String> {
        let bytes = std::fs::read(file_path)?;
        let text = pdf_extract::extract_text_from_mem(&bytes)?;
        Ok(text)
    }

    fn extract_word_text(file_path: &str) -> Result<String> {
        // 读取 docx 文件（本质是 zip 文件）
        let file = std::fs::File::open(file_path)?;
        let mut archive = zip::ZipArchive::new(file)?;

        // 提取 word/document.xml
        let mut document_xml = String::new();
        if let Ok(mut doc_file) = archive.by_name("word/document.xml") {
            doc_file.read_to_string(&mut document_xml)?;
        }

        // 简单提取文本（移除 XML 标签）
        let text = Self::extract_text_from_xml(&document_xml);
        Ok(text)
    }

    fn extract_text_from_xml(xml: &str) -> String {
        // 简单实现：移除 XML 标签，保留文本内容
        let mut result = String::new();
        let mut in_tag = false;
        let mut chars = xml.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '<' => in_tag = true,
                '>' => in_tag = false,
                _ if !in_tag => result.push(c),
                _ => {}
            }
        }

        // 清理多余空白
        result.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    pub fn split_into_paragraphs(text: &str) -> Vec<String> {
        text.split("\n\n")
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}
```

- [ ] **步骤 4：创建 vector/mod.rs**

```rust
pub mod lancedb_store;

pub use lancedb_store::*;
```

- [ ] **步骤 5：创建 vector/lancedb_store.rs**

```rust
use anyhow::Result;
use lancedb::prelude::*;
use std::sync::Arc;

pub struct VectorStore {
    db: Connection,
}

impl VectorStore {
    pub async fn new(db_path: &str) -> Result<Self> {
        let db = connect(db_path).execute().await?;
        Ok(Self { db })
    }

    pub async fn insert_vectors(
        &self,
        table_name: &str,
        ids: &[String],
        texts: &[String],
        embeddings: &[Vec<f32>],
    ) -> Result<()> {
        use lancedb::arrow::array::{Float32Array, StringArray, ArrayRef, RecordBatch};
        use lancedb::arrow::datatypes::{Schema, Field, DataType};

        if embeddings.is_empty() {
            return Ok(());
        }

        let dim = embeddings[0].len() as i32;

        // 创建 schema
        let schema = Arc::new(Schema::new(vec![
            Field::new("id", DataType::Utf8, false),
            Field::new("text", DataType::Utf8, false),
            Field::new("vector", DataType::FixedSizeList(
                Box::new(Field::new("item", DataType::Float32, true)),
                dim,
            ), false),
        ]));

        // 准备数据
        let id_array: ArrayRef = Arc::new(StringArray::from(ids.to_vec()));
        let text_array: ArrayRef = Arc::new(StringArray::from(texts.to_vec()));

        // 展平嵌入向量
        let flat_embeddings: Vec<f32> = embeddings.iter().flatten().copied().collect();
        let vector_array: ArrayRef = Arc::new(
            lancedb::arrow::array::FixedSizeListArray::try_new_from_values(
                Float32Array::from(flat_embeddings),
                dim,
            )?
        );

        let batch = RecordBatch::try_new(schema.clone(), vec![id_array, text_array, vector_array])?;

        // 创建或追加到表
        let table = self.db.create_table(table_name, vec![batch]).execute().await?;
        Ok(())
    }

    pub async fn search(
        &self,
        table_name: &str,
        query_vector: &[f32],
        k: usize,
    ) -> Result<Vec<(String, String, f32)>> {
        let table = self.db.open_table(table_name).execute().await?;

        let results = table
            .vector_search(query_vector.to_vec())?
            .column("vector")?
            .limit(k)
            .execute()
            .await?;

        // 解析结果
        let mut search_results = Vec::new();
        // 遍历结果并提取 id, text, score
        // 实际实现需要根据 LanceDB 返回格式解析
        for batch in results {
            // 解析 Arrow RecordBatch
            // 提取 id, text, _distance 列
        }

        Ok(search_results)
    }

    pub async fn delete_by_document(&self, table_name: &str, document_id: &str) -> Result<()> {
        let table = self.db.open_table(table_name).execute().await?;
        // 删除以 document_id 开头的所有记录
        table.delete(&format!("id LIKE '{}%'", document_id)).await?;
        Ok(())
    }
}
```

- [ ] **步骤 6：Commit**

```bash
git add .
git commit -m "feat: add document commands and vectorization services"
```

---

### 任务 5：实现前端路由和状态管理

**文件：**
- 创建：`src/router/index.ts`
- 创建：`src/stores/index.ts`
- 创建：`src/stores/settings.ts`
- 创建：`src/stores/documents.ts`

- [ ] **步骤 1：创建路由 src/router/index.ts**

```typescript
import { createRouter, createWebHistory } from "vue-router";
import HomeView from "@/views/HomeView.vue";
import SettingsView from "@/views/SettingsView.vue";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "home",
      component: HomeView,
    },
    {
      path: "/settings",
      name: "settings",
      component: SettingsView,
    },
  ],
});

export default router;
```

- [ ] **步骤 2：创建 stores/index.ts**

```typescript
export { useSettingsStore } from "./settings";
export { useDocumentsStore } from "./documents";
```

- [ ] **步骤 3：创建 stores/settings.ts**

```typescript
import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import type { ProviderConfig } from "@/types";

export const useSettingsStore = defineStore("settings", {
  state: () => ({
    providers: [] as ProviderConfig[],
    activeProvider: null as ProviderConfig | null,
    loading: false,
  }),

  actions: {
    async loadProviders() {
      this.loading = true;
      try {
        this.providers = await invoke<ProviderConfig[]>("get_providers");
        this.activeProvider = this.providers.find((p) => p.isActive) || null;
      } finally {
        this.loading = false;
      }
    },

    async saveProvider(provider: ProviderConfig) {
      await invoke("save_provider", { provider });
      await this.loadProviders();
    },

    async testProvider(provider: ProviderConfig): Promise<string> {
      return await invoke<string>("test_provider", { provider });
    },
  },
});
```

- [ ] **步骤 4：创建 stores/documents.ts**

```typescript
import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { Document } from "@/types";

export const useDocumentsStore = defineStore("documents", {
  state: () => ({
    documents: [] as Document[],
    loading: false,
    selectedIds: [] as string[],
  }),

  getters: {
    selectedDocuments: (state) =>
      state.documents.filter((d) => state.selectedIds.includes(d.id)),
  },

  actions: {
    async loadDocuments() {
      this.loading = true;
      try {
        this.documents = await invoke<Document[]>("list_documents");
      } finally {
        this.loading = false;
      }
    },

    async uploadDocument() {
      const selected = await open({
        multiple: false,
        filters: [
          { name: "Documents", extensions: ["pdf", "doc", "docx"] },
        ],
      });

      if (selected) {
        await invoke<Document>("upload_document", { filePath: selected });
        await this.loadDocuments();
      }
    },

    async deleteDocument(id: string) {
      await invoke("delete_document", { id });
      await this.loadDocuments();
    },

    toggleSelection(id: string) {
      const index = this.selectedIds.indexOf(id);
      if (index > -1) {
        this.selectedIds.splice(index, 1);
      } else {
        this.selectedIds.push(id);
      }
    },

    clearSelection() {
      this.selectedIds = [];
    },
  },
});
```

- [ ] **步骤 5：Commit**

```bash
git add .
git commit -m "feat: add Vue Router and Pinia stores"
```

---

### 任务 6：实现设置页面

**文件：**
- 创建：`src/views/SettingsView.vue`
- 创建：`src/components/settings/ProviderConfig.vue`

- [ ] **步骤 1：创建 SettingsView.vue**

```vue
<template>
  <div class="settings-view">
    <el-page-header @back="$router.push('/')">
      <template #content>
        <span class="text-large font-600 mr-3">设置</span>
      </template>
    </el-page-header>

    <div class="settings-content">
      <h3>模型提供商配置</h3>
      <el-tabs v-model="activeTab">
        <el-tab-pane
          v-for="provider in settingsStore.providers"
          :key="provider.id"
          :label="provider.name"
          :name="provider.id"
        >
          <ProviderConfig :provider="provider" />
        </el-tab-pane>
      </el-tabs>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useSettingsStore } from "@/stores/settings";
import ProviderConfig from "@/components/settings/ProviderConfig.vue";

const settingsStore = useSettingsStore();
const activeTab = ref("deepseek");

onMounted(() => {
  settingsStore.loadProviders();
});
</script>

<style scoped>
.settings-view {
  padding: 20px;
  max-width: 800px;
  margin: 0 auto;
}

.settings-content {
  margin-top: 20px;
}

h3 {
  margin-bottom: 16px;
}
</style>
```

- [ ] **步骤 2：创建 ProviderConfig.vue**

```vue
<template>
  <el-form :model="form" label-width="120px" class="provider-form">
    <el-form-item label="API Key">
      <el-input
        v-model="form.apiKey"
        type="password"
        show-password
        placeholder="请输入 API Key"
      />
    </el-form-item>

    <el-form-item label="Base URL">
      <el-input v-model="form.baseUrl" placeholder="API 地址" />
    </el-form-item>

    <el-form-item label="聊天模型">
      <el-input v-model="form.model" placeholder="模型名称" />
    </el-form-item>

    <el-form-item label="嵌入模型">
      <el-input v-model="form.embeddingModel" placeholder="嵌入模型名称" />
    </el-form-item>

    <el-form-item label="启用">
      <el-switch v-model="form.isActive" />
    </el-form-item>

    <el-form-item>
      <el-button @click="handleTest" :loading="testing">
        测试连接
      </el-button>
      <el-button type="primary" @click="handleSave" :loading="saving">
        保存配置
      </el-button>
    </el-form-item>
  </el-form>
</template>

<script setup lang="ts">
import { ref, reactive, watch } from "vue";
import { ElMessage } from "element-plus";
import { useSettingsStore } from "@/stores/settings";
import type { ProviderConfig } from "@/types";

const props = defineProps<{
  provider: ProviderConfig;
}>();

const settingsStore = useSettingsStore();

const form = reactive({
  apiKey: props.provider.apiKey,
  baseUrl: props.provider.baseUrl,
  model: props.provider.model,
  embeddingModel: props.provider.embeddingModel,
  isActive: props.provider.isActive,
});

const testing = ref(false);
const saving = ref(false);

watch(
  () => props.provider,
  (newVal) => {
    form.apiKey = newVal.apiKey;
    form.baseUrl = newVal.baseUrl;
    form.model = newVal.model;
    form.embeddingModel = newVal.embeddingModel;
    form.isActive = newVal.isActive;
  }
);

async function handleTest() {
  if (!form.apiKey) {
    ElMessage.warning("请先输入 API Key");
    return;
  }

  testing.value = true;
  try {
    const result = await settingsStore.testProvider({
      ...props.provider,
      apiKey: form.apiKey,
      baseUrl: form.baseUrl,
      model: form.model,
      embeddingModel: form.embeddingModel,
      isActive: form.isActive,
    });
    ElMessage.success(`连接成功: ${result}`);
  } catch (error: any) {
    ElMessage.error(`连接失败: ${error}`);
  } finally {
    testing.value = false;
  }
}

async function handleSave() {
  saving.value = true;
  try {
    await settingsStore.saveProvider({
      ...props.provider,
      apiKey: form.apiKey,
      baseUrl: form.baseUrl,
      model: form.model,
      embeddingModel: form.embeddingModel,
      isActive: form.isActive,
    });
    ElMessage.success("保存成功");
  } catch (error: any) {
    ElMessage.error(`保存失败: ${error}`);
  } finally {
    saving.value = false;
  }
}
</script>

<style scoped>
.provider-form {
  max-width: 500px;
  margin-top: 20px;
}
</style>
```

- [ ] **步骤 3：Commit**

```bash
git add .
git commit -m "feat: add settings page with provider configuration"
```

---

### 任务 7：实现主布局和文档列表

**文件：**
- 创建：`src/views/HomeView.vue`
- 创建：`src/components/layout/AppLayout.vue`
- 创建：`src/components/documents/DocumentList.vue`
- 创建：`src/components/documents/UploadButton.vue`

- [ ] **步骤 1：创建 AppLayout.vue**

```vue
<template>
  <div class="app-layout">
    <header class="app-header">
      <div class="logo">DocTranslate</div>
      <el-button link @click="$router.push('/settings')">
        <el-icon><Setting /></el-icon>
        设置
      </el-button>
    </header>

    <div class="app-body">
      <aside class="sidebar">
        <slot name="sidebar" />
      </aside>

      <main class="content">
        <slot name="content" />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Setting } from "@element-plus/icons-vue";
</script>

<style scoped>
.app-layout {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.app-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 16px;
  height: 50px;
  border-bottom: 1px solid var(--el-border-color);
  background: var(--el-bg-color);
}

.logo {
  font-size: 18px;
  font-weight: 600;
}

.app-body {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.sidebar {
  width: 280px;
  border-right: 1px solid var(--el-border-color);
  background: var(--el-bg-color-page);
  display: flex;
  flex-direction: column;
}

.content {
  flex: 1;
  overflow: auto;
  padding: 20px;
}
</style>
```

- [ ] **步骤 2：创建 DocumentList.vue**

```vue
<template>
  <div class="document-list">
    <div class="list-header">
      <span>文档列表</span>
      <UploadButton @uploaded="emit('refresh')" />
    </div>

    <el-scrollbar class="list-scroll">
      <div v-if="documentsStore.loading" class="loading">
        <el-icon class="is-loading"><Loading /></el-icon>
      </div>

      <div v-else-if="documentsStore.documents.length === 0" class="empty">
        暂无文档
      </div>

      <div v-else class="document-items">
        <div
          v-for="doc in documentsStore.documents"
          :key="doc.id"
          class="document-item"
          :class="{ selected: isSelected(doc.id) }"
          @click="toggleSelect(doc.id)"
        >
          <el-icon class="doc-icon">
            <Document v-if="doc.type === 'word'" />
            <Tickets v-else />
          </el-icon>

          <div class="doc-info">
            <div class="doc-name">{{ doc.name }}</div>
            <div class="doc-meta">
              <span>{{ formatSize(doc.size) }}</span>
              <el-tag
                size="small"
                :type="doc.vectorized ? 'success' : 'info'"
              >
                {{ doc.vectorized ? "已向量化" : "未向量化" }}
              </el-tag>
            </div>
          </div>

          <el-button
            class="delete-btn"
            type="danger"
            link
            @click.stop="handleDelete(doc.id)"
          >
            <el-icon><Delete /></el-icon>
          </el-button>
        </div>
      </div>
    </el-scrollbar>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from "vue";
import { ElMessageBox } from "element-plus";
import { Document, Tickets, Delete, Loading } from "@element-plus/icons-vue";
import { useDocumentsStore } from "@/stores/documents";
import UploadButton from "./UploadButton.vue";

const emit = defineEmits<{
  refresh: [];
}>();

const documentsStore = useDocumentsStore();

onMounted(() => {
  documentsStore.loadDocuments();
});

function isSelected(id: string) {
  return documentsStore.selectedIds.includes(id);
}

function toggleSelect(id: string) {
  documentsStore.toggleSelection(id);
}

async function handleDelete(id: string) {
  try {
    await ElMessageBox.confirm("确定删除此文档吗？", "提示", {
      type: "warning",
    });
    await documentsStore.deleteDocument(id);
  } catch {
    // 用户取消
  }
}

function formatSize(bytes: number): string {
  if (bytes < 1024) return bytes + " B";
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + " KB";
  return (bytes / (1024 * 1024)).toFixed(1) + " MB";
}
</script>

<style scoped>
.document-list {
  display: flex;
  flex-direction: column;
  height: 100%;
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

.document-items {
  padding: 8px;
}

.document-item {
  display: flex;
  align-items: center;
  padding: 8px;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.document-item:hover {
  background: var(--el-fill-color-light);
}

.document-item.selected {
  background: var(--el-color-primary-light-9);
}

.doc-icon {
  font-size: 24px;
  margin-right: 8px;
  color: var(--el-color-primary);
}

.doc-info {
  flex: 1;
  min-width: 0;
}

.doc-name {
  font-size: 14px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.doc-meta {
  display: flex;
  gap: 8px;
  align-items: center;
  margin-top: 4px;
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.delete-btn {
  opacity: 0;
  transition: opacity 0.2s;
}

.document-item:hover .delete-btn {
  opacity: 1;
}
</style>
```

- [ ] **步骤 3：创建 UploadButton.vue**

```vue
<template>
  <el-button type="primary" size="small" @click="handleUpload">
    <el-icon><Plus /></el-icon>
    上传
  </el-button>
</template>

<script setup lang="ts">
import { Plus } from "@element-plus/icons-vue";
import { useDocumentsStore } from "@/stores/documents";

const emit = defineEmits<{
  uploaded: [];
}>();

const documentsStore = useDocumentsStore();

async function handleUpload() {
  await documentsStore.uploadDocument();
  emit("uploaded");
}
</script>
```

- [ ] **步骤 4：创建 HomeView.vue**

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
            <div class="placeholder">
              <el-empty description="任务模块将在计划 B 中实现" />
            </div>
          </el-tab-pane>

          <el-tab-pane label="聊天" name="chat">
            <div class="placeholder">
              <el-empty description="聊天模块将在计划 B 中实现" />
            </div>
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

const activeTab = ref("tasks");

function handleRefresh() {
  // 刷新内容区域
}
</script>

<style scoped>
.home-content {
  height: 100%;
}

.placeholder {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 400px;
}
</style>
```

- [ ] **步骤 5：Commit**

```bash
git add .
git commit -m "feat: add main layout and document list component"
```

---

### 任务 8：集成测试和构建验证

- [ ] **步骤 1：安装 Tauri CLI**

运行：
```bash
npm install
```

- [ ] **步骤 2：验证前端构建**

运行：
```bash
npm run build
```
预期：构建成功，生成 dist 目录

- [ ] **步骤 3：验证 Tauri 开发模式**

运行：
```bash
npm run tauri dev
```
预期：应用启动，显示主界面

- [ ] **步骤 4：测试设置功能**

手动测试：
1. 进入设置页面
2. 填写 API Key
3. 点击测试连接
4. 保存配置

- [ ] **步骤 5：测试文档上传**

手动测试：
1. 点击上传按钮
2. 选择 PDF/Word 文件
3. 确认文件出现在列表中
4. 测试删除功能

- [ ] **步骤 6：Commit**

```bash
git add .
git commit -m "test: verify build and basic functionality"
```

---

## 检查点

完成此计划后，应该得到：

- [x] 可运行的 Tauri 应用
- [x] 设置页面可配置 AI 提供商
- [x] 文档管理：上传、列表、删除
- [x] 前后端通过 Tauri IPC 通信正常
- [x] 数据持久化（SQLite + 配置文件）

**下一步：** 计划 B 将实现 toPdf 任务和聊天模块。