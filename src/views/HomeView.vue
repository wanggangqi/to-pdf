<template>
  <AppLayout>
    <template #sidebar>
      <DocumentList @refresh="handleRefresh" />
    </template>

    <template #content>
      <div class="home-content">
        <el-tabs v-model="activeTab">
          <el-tab-pane label="toPdf 任务" name="tasks">
            <div class="placeholder">
              <el-empty description="任务模块将在计划 B 中实现" />
            </div>
          </el-tab-pane>

          <el-tab-pane label="聊天" name="chat">
            <div class="placeholder">
              <el-empty description="聊天模块将在计划 B 中实现" />
            </div>
          </el-tab-pane>
        </el-tabs>
      </div>
    </template>
  </AppLayout>
</template>

<script setup lang="ts">
import { ref } from "vue";
import AppLayout from "@/components/layout/AppLayout.vue";
import DocumentList from "@/components/documents/DocumentList.vue";

const activeTab = ref("tasks");

function handleRefresh() {
  // 刷新内容区域
}
</script>

<style scoped>
.home-content {
  height: 100%;
}

.placeholder {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 400px;
}
</style>