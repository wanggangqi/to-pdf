<template>
  <div class="chat-view">
    <div class="chat-header">
      <span>智能问答</span>
      <el-select
        v-model="selectedDocIds"
        multiple
        collapse-tags
        collapse-tags-tooltip
        placeholder="选择文档"
        size="small"
        style="width: 200px"
        @change="handleDocSelect"
      >
        <el-option
          v-for="doc in documentsStore.documents.filter((d) => d.vectorized)"
          :key="doc.id"
          :label="doc.name"
          :value="doc.id"
        />
      </el-select>
    </div>

    <MessageList :messages="chatStore.messages" :loading="chatStore.loading" />

    <ChatInput
      :disabled="chatStore.loading"
      :placeholder="inputPlaceholder"
      @send="handleSend"
    />

    <div v-if="chatStore.messages.length > 0" class="chat-footer">
      <el-button size="small" text type="info" @click="handleClear">
        清空对话
      </el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useChatStore } from "@/stores/chat";
import { useDocumentsStore } from "@/stores/documents";
import { useApiKeyCheck } from "@/composables/useApiKeyCheck";
import MessageList from "./MessageList.vue";
import ChatInput from "./ChatInput.vue";

const chatStore = useChatStore();
const documentsStore = useDocumentsStore();
const { checkApiKey } = useApiKeyCheck();

const selectedDocIds = ref<string[]>([]);

const inputPlaceholder = computed(() => {
  if (selectedDocIds.value.length === 0) {
    return "请先选择文档...";
  }
  return "输入问题，按 Ctrl+Enter 发送";
});

onMounted(() => {
  documentsStore.loadDocuments();
});

function handleDocSelect(ids: string[]) {
  chatStore.setSelectedDocuments(ids);
}

async function handleSend(content: string) {
  if (selectedDocIds.value.length === 0) {
    return;
  }
  if (await checkApiKey()) {
    await chatStore.sendMessage(content);
  }
}

function handleClear() {
  chatStore.clearMessages();
}
</script>

<style scoped>
.chat-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--el-bg-color);
}

.chat-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid var(--el-border-color-light);
  font-weight: 600;
}

.chat-footer {
  display: flex;
  justify-content: center;
  padding: 8px;
  border-top: 1px solid var(--el-border-color-lighter);
}
</style>