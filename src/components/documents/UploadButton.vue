<template>
  <el-button type="primary" size="small" @click="handleUpload">
    <el-icon><Plus /></el-icon>
    上传
  </el-button>
</template>

<script setup lang="ts">
import { Plus } from "@element-plus/icons-vue";
import { useDocumentsStore } from "@/stores/documents";
import { useApiKeyCheck } from "@/composables/useApiKeyCheck";

const emit = defineEmits<{
  uploaded: [];
}>();

const documentsStore = useDocumentsStore();
const { checkApiKey } = useApiKeyCheck();

async function handleUpload() {
  if (!await checkApiKey()) return;
  await documentsStore.uploadDocument();
  emit("uploaded");
}
</script>