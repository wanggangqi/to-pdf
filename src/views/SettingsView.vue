<template>
  <div class="settings-view">
    <el-page-header @back="$router.push('/')">
      <template #content>
        <span class="text-large font-600 mr-3">设置</span>
      </template>
    </el-page-header>

    <div class="settings-content">
      <h3>模型提供商配置</h3>
      <el-tabs v-model="activeTab">
        <el-tab-pane
          v-for="provider in settingsStore.providers"
          :key="provider.id"
          :label="provider.name"
          :name="provider.id"
        >
          <ProviderConfig :provider="provider" />
        </el-tab-pane>
      </el-tabs>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useSettingsStore } from "@/stores/settings";
import ProviderConfig from "@/components/settings/ProviderConfig.vue";

const settingsStore = useSettingsStore();
const activeTab = ref("deepseek");

onMounted(() => {
  settingsStore.loadProviders();
});
</script>

<style scoped>
.settings-view {
  padding: 20px;
  max-width: 800px;
  margin: 0 auto;
}

.settings-content {
  margin-top: 20px;
}

h3 {
  margin-bottom: 16px;
}
</style>