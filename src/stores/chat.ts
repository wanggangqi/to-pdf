import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import type { ChatMessage } from "@/types";

export const useChatStore = defineStore("chat", {
  state: () => ({
    messages: [] as ChatMessage[],
    loading: false,
    selectedDocumentIds: [] as string[],
  }),

  actions: {
    async sendMessage(content: string) {
      const userMessage: ChatMessage = {
        id: Date.now().toString(),
        role: "user",
        content,
        createdAt: new Date().toISOString(),
      };
      this.messages.push(userMessage);
      this.loading = true;

      try {
        const response = await invoke<string>("chat", {
          message: content,
          documentIds: this.selectedDocumentIds,
        });

        const assistantMessage: ChatMessage = {
          id: (Date.now() + 1).toString(),
          role: "assistant",
          content: response,
          createdAt: new Date().toISOString(),
        };
        this.messages.push(assistantMessage);
      } catch (error: any) {
        const errorMessage: ChatMessage = {
          id: (Date.now() + 1).toString(),
          role: "assistant",
          content: `错误: ${error}`,
          createdAt: new Date().toISOString(),
        };
        this.messages.push(errorMessage);
      } finally {
        this.loading = false;
      }
    },

    clearMessages() {
      this.messages = [];
    },

    setSelectedDocuments(ids: string[]) {
      this.selectedDocumentIds = ids;
    },
  },
});