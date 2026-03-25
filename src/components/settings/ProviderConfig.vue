<template>
  <div class="provider-config">
    <el-card v-for="provider in providers" :key="provider.id" class="provider-card">
      <template #header>
        <div class="card-header">
          <span>{{ provider.name }}</span>
          <el-switch v-model="provider.enabled" />
        </div>
      </template>
      <el-form label-width="100px">
        <el-form-item label="API Key">
          <el-input v-model="provider.apiKey" type="password" show-password />
        </el-form-item>
        <el-form-item label="Base URL">
          <el-input v-model="provider.baseUrl" />
        </el-form-item>
        <el-form-item label="模型">
          <el-input v-model="provider.model" />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="handleTest(provider.id)">
            测试连接
          </el-button>
          <el-button @click="handleSave(provider)">保存</el-button>
        </el-form-item>
      </el-form>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

interface Provider {
  id: string;
  name: string;
  apiKey: string;
  baseUrl: string;
  model: string;
  enabled: boolean;
}

const providers = ref<Provider[]>([
  { id: "deepseek", name: "DeepSeek", apiKey: "", baseUrl: "https://api.deepseek.com", model: "deepseek-chat", enabled: false },
  { id: "moonshot", name: "Moonshot", apiKey: "", baseUrl: "https://api.moonshot.cn", model: "moonshot-v1-8k", enabled: false },
  { id: "zhipu", name: "智谱", apiKey: "", baseUrl: "https://open.bigmodel.cn", model: "glm-4", enabled: false },
  { id: "bailian", name: "百炼", apiKey: "", baseUrl: "https://dashscope.aliyuncs.com", model: "qwen-turbo", enabled: false },
]);

function handleTest(providerId: string) {
  console.log("Testing provider:", providerId);
  // TODO: Implement with Tauri backend
}

function handleSave(provider: Provider) {
  console.log("Saving provider:", provider);
  // TODO: Implement with Tauri backend
}
</script>

<style scoped>
.provider-config {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.provider-card {
  max-width: 600px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
</style>