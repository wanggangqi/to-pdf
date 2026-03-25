<template>
  <div class="chat-input">
    <el-input
      v-model="inputText"
      type="textarea"
      :rows="3"
      :placeholder="placeholder"
      :disabled="disabled"
      resize="none"
      @keydown="handleKeydown"
    />
    <div class="input-actions">
      <span class="hint">Ctrl + Enter 发送</span>
      <el-button
        type="primary"
        :disabled="!canSend"
        :loading="disabled"
        @click="handleSend"
      >
        发送
      </el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";

const props = defineProps<{
  disabled?: boolean;
  placeholder?: string;
}>();

const emit = defineEmits<{
  send: [content: string];
}>();

const inputText = ref("");

const canSend = computed(() => {
  return inputText.value.trim().length > 0 && !props.disabled;
});

function handleKeydown(event: KeyboardEvent) {
  // Ctrl + Enter 发送
  if (event.ctrlKey && event.key === "Enter") {
    event.preventDefault();
    handleSend();
  }
}

function handleSend() {
  const content = inputText.value.trim();
  if (content && !props.disabled) {
    emit("send", content);
    inputText.value = "";
  }
}
</script>

<style scoped>
.chat-input {
  padding: 12px 16px;
  border-top: 1px solid var(--el-border-color-light);
  background: var(--el-bg-color);
}

.chat-input .el-textarea {
  margin-bottom: 8px;
}

.input-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.hint {
  font-size: 12px;
  color: var(--el-text-color-secondary);
}
</style>