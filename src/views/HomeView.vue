<template>
  <AppLayout>
    <template #sidebar>
      <DocumentList @refresh="handleRefresh" />
    </template>

    <template #content>
      <div class="home-content">
        <el-tabs v-model="activeTab">
          <el-tab-pane label="toPdf 任务" name="tasks">
            <TaskList />
          </el-tab-pane>

          <el-tab-pane label="聊天" name="chat">
            <ChatView />
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
import TaskList from "@/components/tasks/TaskList.vue";
import ChatView from "@/components/chat/ChatView.vue";

const activeTab = ref("tasks");

function handleRefresh() {
  // 刷新内容区域
}
</script>

<style scoped>
.home-content {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.home-content :deep(.el-tabs__content) {
  flex: 1;
  overflow: hidden;
}

.home-content :deep(.el-tab-pane) {
  height: 100%;
}
</style>