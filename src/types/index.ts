// AI Provider types
export interface ProviderConfig {
  id: string;
  name: string;
  apiKey: string;
  baseUrl: string;
  model: string;
  enabled: boolean;
}

// Document types
export interface Document {
  id: string;
  filename: string;
  fileType: "pdf" | "docx";
  fileSize: number;
  createdAt: string;
  status: "pending" | "processing" | "completed" | "error";
}

// Translation task types
export interface TranslationTask {
  id: string;
  documentId: string;
  status: "pending" | "processing" | "completed" | "error";
  progress: number;
  createdAt: string;
  completedAt?: string;
}

// Chat types
export interface ChatMessage {
  id: string;
  role: "user" | "assistant";
  content: string;
  createdAt: string;
}

export interface ChatSession {
  id: string;
  documentIds: string[];
  messages: ChatMessage[];
  createdAt: string;
}