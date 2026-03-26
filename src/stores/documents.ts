import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/plugin-dialog";
import type { Document } from "@/types";

interface VectorizeProgress {
  documentId: string;
  status: "started" | "progress" | "completed" | "failed";
  batch?: number;
  totalBatches?: number;
  total?: number;
  error?: string;
}

export const useDocumentsStore = defineStore("documents", {
  state: () => ({
    documents: [] as Document[],
    loading: false,
    selectedIds: [] as string[],
    vectorizingIds: [] as string[],
    vectorizeProgress: {} as Record<string, { batch: number; totalBatches: number }>,
    unlisten: null as UnlistenFn | null,
  }),

  getters: {
    selectedDocuments: (state) =>
      state.documents.filter((d) => state.selectedIds.includes(d.id)),
    getVectorizeProgress: (state) => (id: string) => state.vectorizeProgress[id],
  },

  actions: {
    async setupListeners() {
      if (this.unlisten) return;

      this.unlisten = await listen<VectorizeProgress>("vectorize-progress", (event) => {
        const { documentId, status, batch, totalBatches } = event.payload;

        if (status === "started") {
          this.vectorizeProgress[documentId] = { batch: 0, totalBatches: 0 };
        } else if (status === "progress" && batch && totalBatches) {
          this.vectorizeProgress[documentId] = { batch, totalBatches };
        } else if (status === "completed" || status === "failed") {
          delete this.vectorizeProgress[documentId];
        }
      });
    },

    async loadDocuments() {
      this.loading = true;
      try {
        this.documents = await invoke<Document[]>("list_documents");
      } finally {
        this.loading = false;
      }
    },

    async uploadDocument() {
      const selected = await open({
        multiple: false,
        filters: [
          { name: "Documents", extensions: ["pdf", "doc", "docx"] },
        ],
      });

      if (selected) {
        const doc = await invoke<Document>("upload_document", { filePath: selected });
        await this.loadDocuments();

        // 自动触发向量化
        if (doc && doc.id) {
          await this.vectorizeDocument(doc.id);
        }
      }
    },

    async deleteDocument(id: string) {
      await invoke("delete_document", { id });
      await this.loadDocuments();
    },

    async vectorizeDocument(id: string) {
      if (this.vectorizingIds.includes(id)) return;

      // 确保监听器已设置
      await this.setupListeners();

      this.vectorizingIds.push(id);
      this.vectorizeProgress[id] = { batch: 0, totalBatches: 0 };

      try {
        await invoke("vectorize_document", { documentId: id });
        await this.loadDocuments();
      } catch (error) {
        console.error("Vectorization failed:", error);
        throw error;
      } finally {
        const index = this.vectorizingIds.indexOf(id);
        if (index > -1) {
          this.vectorizingIds.splice(index, 1);
        }
        delete this.vectorizeProgress[id];
      }
    },

    toggleSelection(id: string) {
      const index = this.selectedIds.indexOf(id);
      if (index > -1) {
        this.selectedIds.splice(index, 1);
      } else {
        this.selectedIds.push(id);
      }
    },

    clearSelection() {
      this.selectedIds = [];
    },
  },
});