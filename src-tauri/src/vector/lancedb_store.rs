use anyhow::Result;
use arrow::{
    array::{FixedSizeListArray, StringArray, Float32Array, RecordBatch},
    datatypes::{DataType, Field, Schema, Float32Type},
    record_batch::RecordBatchIterator,
};
use lancedb::{
    connect,
    query::{ExecutableQuery, QueryBase},
};
use std::sync::Arc;
use futures::StreamExt;

/// LanceDB vector store for document embeddings
pub struct VectorStore {
    db_path: String,
}

impl VectorStore {
    /// 创建新的向量存储实例
    pub fn new(db_path: &str) -> Self {
        Self {
            db_path: db_path.to_string(),
        }
    }

    /// 获取 Arrow Schema（动态维度）
    fn schema_with_dim(dim: i32) -> Arc<Schema> {
        Arc::new(Schema::new(vec![
            Field::new("id", DataType::Utf8, false),
            Field::new("document_id", DataType::Utf8, false),
            Field::new("text", DataType::Utf8, false),
            Field::new(
                "vector",
                DataType::FixedSizeList(
                    Arc::new(Field::new("item", DataType::Float32, true)),
                    dim,
                ),
                false,
            ),
        ]))
    }

    /// 初始化向量存储，创建表
    pub async fn init(&self) -> Result<()> {
        let db = connect(&self.db_path).execute().await?;

        // 检查表是否存在
        let table_names = db.table_names().execute().await?;
        if !table_names.contains(&"documents".to_string()) {
            // 创建空表（使用默认维度 1024）
            let schema = Self::schema_with_dim(1024);
            let empty_batch = RecordBatch::new_empty(schema.clone());
            let reader = RecordBatchIterator::new(vec![Ok(empty_batch)], schema);

            db.create_table("documents", Box::new(reader))
                .execute()
                .await?;
        }

        Ok(())
    }

    /// 插入向量和文本
    /// - ids: 段落 ID 列表
    /// - document_id: 文档 ID
    /// - texts: 段落文本列表
    /// - embeddings: 嵌入向量列表
    pub async fn insert(
        &self,
        ids: &[String],
        document_id: &str,
        texts: &[String],
        embeddings: &[Vec<f32>],
    ) -> Result<()> {
        if ids.is_empty() || embeddings.is_empty() {
            return Ok(());
        }

        // 动态获取向量维度
        let embedding_dim = embeddings[0].len() as i32;
        println!("[DEBUG] Inserting vectors with dimension: {}", embedding_dim);

        let db = connect(&self.db_path).execute().await?;

        // 确保表存在
        let table_names = db.table_names().execute().await?;
        if !table_names.contains(&"documents".to_string()) {
            // 创建表时使用实际的向量维度
            let schema = Self::schema_with_dim(embedding_dim);
            let empty_batch = RecordBatch::new_empty(schema.clone());
            let reader = RecordBatchIterator::new(vec![Ok(empty_batch)], schema);

            db.create_table("documents", Box::new(reader))
                .execute()
                .await?;
        } else {
            // 检查现有表的维度
            let table = db.open_table("documents").execute().await?;
            let schema = table.schema().await?;

            // 查找 vector 字段
            for field in schema.fields() {
                if field.name() == "vector" {
                    if let DataType::FixedSizeList(_, dim) = field.data_type() {
                        if *dim != embedding_dim {
                            return Err(anyhow::anyhow!(
                                "向量维度不匹配：现有数据库维度为 {}，当前模型输出维度为 {}。\n\
                                 解决方法：删除应用数据目录下的 vectors.db 文件，然后重新向量化文档。",
                                dim, embedding_dim
                            ));
                        }
                    }
                    break;
                }
            }
        }

        let table = db.open_table("documents").execute().await?;

        // 构建 RecordBatch（使用实际维度）
        let schema = Self::schema_with_dim(embedding_dim);
        let batch = self.build_batch(ids, document_id, texts, embeddings, schema.clone(), embedding_dim)?;

        // 添加数据
        let reader = RecordBatchIterator::new(vec![Ok(batch)], schema);
        table.add(Box::new(reader)).execute().await?;

        Ok(())
    }

    /// 构建 RecordBatch
    fn build_batch(
        &self,
        ids: &[String],
        document_id: &str,
        texts: &[String],
        embeddings: &[Vec<f32>],
        schema: Arc<Schema>,
        embedding_dim: i32,
    ) -> Result<RecordBatch> {
        let n = ids.len();

        // ID 列
        let id_array = StringArray::from(ids.to_vec());

        // Document ID 列（所有行相同）
        let doc_id_array = StringArray::from(vec![document_id.to_string(); n]);

        // Text 列
        let text_array = StringArray::from(texts.to_vec());

        // Vector 列
        let vector_array = FixedSizeListArray::from_iter_primitive::<Float32Type, _, _>(
            embeddings
                .iter()
                .map(|v| Some(v.iter().map(|&f| Some(f)).collect::<Vec<_>>())),
            embedding_dim,
        );

        let batch = RecordBatch::try_new(
            schema,
            vec![
                Arc::new(id_array) as _,
                Arc::new(doc_id_array) as _,
                Arc::new(text_array) as _,
                Arc::new(vector_array) as _,
            ],
        )?;

        Ok(batch)
    }

    /// 向量相似性搜索
    /// 返回 (id, text, distance) 元组列表
    pub async fn search(&self, query: &[f32], limit: usize) -> Result<Vec<(String, String, f32)>> {
        let db = connect(&self.db_path).execute().await?;

        let table_names = db.table_names().execute().await?;
        if !table_names.contains(&"documents".to_string()) {
            return Ok(vec![]);
        }

        let table = db.open_table("documents").execute().await?;

        // 执行向量搜索
        let mut results_stream = table
            .query()
            .nearest_to(query)?
            .limit(limit)
            .execute()
            .await?;

        let mut results = Vec::new();

        // 遍历结果流
        while let Some(batch) = results_stream.next().await {
            let batch = batch?;

            if batch.num_rows() == 0 {
                continue;
            }

            // 获取各列
            let id_col = batch
                .column_by_name("id")
                .and_then(|c| c.as_any().downcast_ref::<StringArray>())
                .ok_or_else(|| anyhow::anyhow!("Failed to get id column"))?;

            let text_col = batch
                .column_by_name("text")
                .and_then(|c| c.as_any().downcast_ref::<StringArray>())
                .ok_or_else(|| anyhow::anyhow!("Failed to get text column"))?;

            let distance_col = batch
                .column_by_name("_distance")
                .and_then(|c| c.as_any().downcast_ref::<Float32Array>());

            for i in 0..batch.num_rows() {
                let id = id_col.value(i).to_string();
                let text = text_col.value(i).to_string();
                let distance = distance_col
                    .map(|d| d.value(i))
                    .unwrap_or(0.0);
                results.push((id, text, distance));
            }
        }

        Ok(results)
    }

    /// 删除指定文档的所有向量
    pub async fn delete(&self, document_id: &str) -> Result<()> {
        let db = connect(&self.db_path).execute().await?;

        let table_names = db.table_names().execute().await?;
        if !table_names.contains(&"documents".to_string()) {
            return Ok(());
        }

        let table = db.open_table("documents").execute().await?;
        table.delete(&format!("document_id = '{}'", document_id)).await?;

        Ok(())
    }

    /// 获取表中向量数量
    pub async fn count(&self) -> Result<usize> {
        let db = connect(&self.db_path).execute().await?;

        let table_names = db.table_names().execute().await?;
        if !table_names.contains(&"documents".to_string()) {
            return Ok(0);
        }

        let table = db.open_table("documents").execute().await?;
        let count = table.count_rows(None).await?;

        Ok(count)
    }
}