<template>
  <div class="upload-button">
    <el-button type="primary" @click="handleUpload">
      <el-icon><Upload /></el-icon>
      上传文档
    </el-button>
    <input
      ref="fileInput"
      type="file"
      accept=".pdf,.docx"
      style="display: none"
      @change="handleFileChange"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { Upload } from "@element-plus/icons-vue";

const fileInput = ref<HTMLInputElement | null>(null);

function handleUpload() {
  fileInput.value?.click();
}

async function handleFileChange(event: Event) {
  const target = event.target as HTMLInputElement;
  const file = target.files?.[0];
  if (file) {
    console.log("Selected file:", file.name);
    // TODO: Implement upload via Tauri backend
  }
  // Reset input
  if (fileInput.value) {
    fileInput.value.value = "";
  }
}
</script>

<style scoped>
.upload-button {
  display: inline-block;
}
</style>