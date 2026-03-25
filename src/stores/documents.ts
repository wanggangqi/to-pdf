import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { Document } from "@/types";

export const useDocumentsStore = defineStore("documents", {
  state: () => ({
    documents: [] as Document[],
    loading: false,
    selectedIds: [] as string[],
  }),

  getters: {
    selectedDocuments: (state) =>
      state.documents.filter((d) => state.selectedIds.includes(d.id)),
  },

  actions: {
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
        await invoke<Document>("upload_document", { filePath: selected });
        await this.loadDocuments();
      }
    },

    async deleteDocument(id: string) {
      await invoke("delete_document", { id });
      await this.loadDocuments();
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