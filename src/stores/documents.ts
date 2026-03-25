import { defineStore } from "pinia";
import { ref } from "vue";

export interface Document {
  id: string;
  filename: string;
  fileType: "pdf" | "docx";
  fileSize: number;
  createdAt: string;
  status: "pending" | "processing" | "completed" | "error";
}

export const useDocumentsStore = defineStore("documents", () => {
  const documents = ref<Document[]>([]);
  const loading = ref(false);

  async function loadDocuments() {
    // TODO: Implement loading from Tauri backend
    loading.value = true;
    try {
      // Placeholder for Tauri invoke
    } finally {
      loading.value = false;
    }
  }

  async function uploadDocument(file: File) {
    // TODO: Implement upload via Tauri backend
    console.log("Uploading document:", file.name);
  }

  async function deleteDocument(id: string) {
    // TODO: Implement delete via Tauri backend
    console.log("Deleting document:", id);
  }

  return {
    documents,
    loading,
    loadDocuments,
    uploadDocument,
    deleteDocument,
  };
});