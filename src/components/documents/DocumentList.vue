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
                :type="getVectorizedType(doc)"
              >
                {{ getVectorizedText(doc) }}
              </el-tag>
            </div>
          </div>

          <div class="doc-actions">
            <template v-if="!doc.vectorized && !isVectorizing(doc.id)">
              <el-button
                type="warning"
                link
                size="small"
                @click.stop="handleVectorize(doc.id)"
              >
                向量化
              </el-button>
            </template>
            <template v-else-if="isVectorizing(doc.id)">
              <div class="vectorizing-progress">
                <el-progress
                  type="circle"
                  :width="24"
                  :stroke-width="3"
                  :percentage="getProgressPercent(doc.id)"
                  :show-text="false"
                />
                <span class="progress-text">{{ getProgressText(doc.id) }}</span>
              </div>
            </template>
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
      </div>
    </el-scrollbar>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from "vue";
import { ElMessageBox, ElMessage } from "element-plus";
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

function isVectorizing(id: string) {
  return documentsStore.vectorizingIds.includes(id);
}

function getProgressPercent(id: string): number {
  const progress = documentsStore.getVectorizeProgress(id);
  if (!progress || progress.totalBatches === 0) return 0;
  return Math.round((progress.batch / progress.totalBatches) * 100);
}

function getProgressText(id: string): string {
  const progress = documentsStore.getVectorizeProgress(id);
  if (!progress || progress.totalBatches === 0) return "准备中...";
  return `${progress.batch}/${progress.totalBatches}`;
}

function getVectorizedType(doc: { vectorized: boolean }) {
  if (doc.vectorized) return "success";
  return "info";
}

function getVectorizedText(doc: { vectorized: boolean }) {
  if (doc.vectorized) return "已向量化";
  return "未向量化";
}

async function handleVectorize(id: string) {
  try {
    await documentsStore.vectorizeDocument(id);
    ElMessage.success("向量化完成");
  } catch (error: any) {
    ElMessage.error(`向量化失败: ${error}`);
  }
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

.doc-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.vectorizing-progress {
  display: flex;
  align-items: center;
  gap: 6px;
}

.progress-text {
  font-size: 12px;
  color: var(--el-text-color-secondary);
  white-space: nowrap;
}
</style>