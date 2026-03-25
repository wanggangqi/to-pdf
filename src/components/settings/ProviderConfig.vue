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