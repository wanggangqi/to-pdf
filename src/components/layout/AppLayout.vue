<template>
  <div class="app-layout">
    <header class="app-header">
      <div class="logo">DocTranslate</div>
      <el-button link @click="$router.push('/settings')">
        <el-icon><Setting /></el-icon>
        设置
      </el-button>
    </header>

    <div class="app-body">
      <aside class="sidebar">
        <slot name="sidebar" />
      </aside>

      <main class="content">
        <slot name="content" />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Setting } from "@element-plus/icons-vue";
</script>

<style scoped>
.app-layout {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.app-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 16px;
  height: 50px;
  border-bottom: 1px solid var(--el-border-color);
  background: var(--el-bg-color);
}

.logo {
  font-size: 18px;
  font-weight: 600;
}

.app-body {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.sidebar {
  width: 280px;
  border-right: 1px solid var(--el-border-color);
  background: var(--el-bg-color-page);
  display: flex;
  flex-direction: column;
}

.content {
  flex: 1;
  overflow: auto;
  padding: 20px;
}
</style>