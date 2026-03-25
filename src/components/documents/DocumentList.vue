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