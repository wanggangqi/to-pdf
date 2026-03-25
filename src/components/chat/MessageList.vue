<template>
  <el-scrollbar ref="scrollbarRef" class="message-list">
    <div class="messages-container">
      <div v-if="messages.length === 0" class="empty-chat">
        <el-icon :size="48"><ChatDotRound /></el-icon>
        <p>选择文档后开始对话</p>
        <p class="hint">基于文档内容的智能问答</p>
      </div>

      <MessageItem
        v-for="message in messages"
        :key="message.id"
        :message="message"
      />

      <div v-if="loading" class="loading-message">
        <div class="assistant-avatar">
          <el-icon><Service /></el-icon>
        </div>
        <div class="loading-content">
          <el-icon class="is-loading"><Loading /></el-icon>
          <span>思考中...</span>
        </div>
      </div>
    </div>
  </el-scrollbar>
</template>

<script setup lang="ts">
import { ref, watch, nextTick } from "vue";
import { ChatDotRound, Service, Loading } from "@element-plus/icons-vue";
import type { ChatMessage } from "@/types";
import MessageItem from "./MessageItem.vue";

const props = defineProps<{
  messages: ChatMessage[];
  loading: boolean;
}>();

const scrollbarRef = ref();

// 消息更新时自动滚动到底部
watch(
  () => props.messages.length,
  () => {
    nextTick(() => {
      scrollToBottom();
    });
  }
);

watch(
  () => props.loading,
  () => {
    nextTick(() => {
      scrollToBottom();
    });
  }
);

function scrollToBottom() {
  if (scrollbarRef.value) {
    scrollbarRef.value.setScrollTop(999999);
  }
}
</script>

<style scoped>
.message-list {
  flex: 1;
  overflow: hidden;
}

.messages-container {
  padding: 16px;
  min-height: 100%;
}

.empty-chat {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 300px;
  color: var(--el-text-color-secondary);
}

.empty-chat p {
  margin: 8px 0 0;
}

.empty-chat .hint {
  font-size: 12px;
  opacity: 0.7;
}

.loading-message {
  display: flex;
  gap: 12px;
  margin-top: 12px;
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

.loading-content {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 16px;
  background: var(--el-fill-color-light);
  border-radius: 12px;
  font-size: 14px;
  color: var(--el-text-color-secondary);
}
</style>