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