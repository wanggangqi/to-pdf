<template>
  <div class="settings-view">
    <el-page-header @back="$router.back()">
      <template #content>
        <span class="text-large font-600 mr-3">设置</span>
      </template>
    </el-page-header>

    <div class="settings-content">
      <h3>AI 提供商配置</h3>
      <ProviderConfig />
    </div>
  </div>
</template>

<script setup lang="ts">
import ProviderConfig from "@/components/settings/ProviderConfig.vue";
</script>

<style scoped>
.settings-view {
  padding: 20px;
  height: 100%;
}

.settings-content {
  margin-top: 20px;
}
</style>