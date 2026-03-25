<template>
  <div class="message-item" :class="message.role">
    <div v-if="message.role === 'user'" class="user-avatar">
      <el-icon><User /></el-icon>
    </div>
    <div v-else class="assistant-avatar">
      <el-icon><Service /></el-icon>
    </div>

    <div class="message-content">
      <div class="message-header">
        <span class="role-name">{{ message.role === 'user' ? '你' : '助手' }}</span>
        <span class="message-time">{{ formatTime(message.createdAt) }}</span>
      </div>
      <div class="message-text" v-html="formattedContent"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { User, Service } from "@element-plus/icons-vue";
import type { ChatMessage } from "@/types";

const props = defineProps<{
  message: ChatMessage;
}>();

const formattedContent = computed(() => {
  // 简单的换行处理
  return props.message.content
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/\n/g, "<br>");
});

function formatTime(isoString: string): string {
  const date = new Date(isoString);
  return date.toLocaleString("zh-CN", {
    hour: "2-digit",
    minute: "2-digit",
  });
}
</script>

<style scoped>
.message-item {
  display: flex;
  gap: 12px;
  margin-bottom: 16px;
}

.message-item.user {
  flex-direction: row-reverse;
}

.user-avatar {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  background: var(--el-color-primary);
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  flex-shrink: 0;
}

.assistant-avatar {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  background: var(--el-color-primary-light-8);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--el-color-primary);
  flex-shrink: 0;
}

.message-content {
  max-width: 70%;
  min-width: 100px;
}

.message-item.user .message-content {
  align-items: flex-end;
}

.message-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 4px;
}

.message-item.user .message-header {
  flex-direction: row-reverse;
}

.role-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--el-text-color-primary);
}

.message-time {
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.message-text {
  padding: 12px 16px;
  border-radius: 12px;
  font-size: 14px;
  line-height: 1.6;
  word-break: break-word;
}

.message-item.user .message-text {
  background: var(--el-color-primary);
  color: white;
  border-bottom-right-radius: 4px;
}

.message-item.assistant .message-text {
  background: var(--el-fill-color-light);
  color: var(--el-text-color-primary);
  border-bottom-left-radius: 4px;
}
</style>