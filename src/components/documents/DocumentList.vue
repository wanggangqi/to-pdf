<template>
  <div class="document-list">
    <el-table :data="documents" style="width: 100%">
      <el-table-column prop="filename" label="文件名" />
      <el-table-column prop="fileType" label="类型" width="100" />
      <el-table-column prop="fileSize" label="大小" width="120">
        <template #default="{ row }">
          {{ formatFileSize(row.fileSize) }}
        </template>
      </el-table-column>
      <el-table-column prop="status" label="状态" width="100">
        <template #default="{ row }">
          <el-tag :type="getStatusType(row.status)">
            {{ row.status }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column prop="createdAt" label="创建时间" width="180" />
      <el-table-column label="操作" width="100">
        <template #default="{ row }">
          <el-button type="danger" size="small" @click="handleDelete(row.id)">
            删除
          </el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

interface Document {
  id: string;
  filename: string;
  fileType: string;
  fileSize: number;
  status: string;
  createdAt: string;
}

const documents = ref<Document[]>([]);

function formatFileSize(bytes: number): string {
  if (bytes < 1024) return bytes + " B";
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + " KB";
  return (bytes / (1024 * 1024)).toFixed(1) + " MB";
}

function getStatusType(status: string): string {
  switch (status) {
    case "completed": return "success";
    case "processing": return "warning";
    case "error": return "danger";
    default: return "info";
  }
}

function handleDelete(id: string) {
  console.log("Deleting document:", id);
  // TODO: Implement with Tauri backend
}
</script>

<style scoped>
.document-list {
  width: 100%;
}
</style>