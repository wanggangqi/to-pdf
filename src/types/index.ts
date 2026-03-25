// 模型提供商配置
export interface ProviderConfig {
  id: string; // deepseek | moonshot | zhipu | bailian
  name: string;
  apiKey: string;
  baseUrl: string;
  model: string;
  embeddingModel: string;
  isActive: boolean;
}

// 文档
export interface Document {
  id: string;
  name: string;
  type: "word" | "pdf";
  size: number;
  path: string;
  createdAt: string;
  vectorized: boolean;
}

// 任务
export interface Task {
  id: string;
  documentId: string;
  status: "pending" | "processing" | "completed" | "failed";
  progress: number;
  outputPath?: string;
  error?: string;
  createdAt: string;
  completedAt?: string;
}

// 聊天消息
export interface ChatMessage {
  id: string;
  role: "user" | "assistant";
  content: string;
  createdAt: string;
}

// 提供商预设
export const PROVIDER_PRESETS: Record<string, Partial<ProviderConfig>> = {
  deepseek: {
    id: "deepseek",
    name: "DeepSeek",
    baseUrl: "https://api.deepseek.com",
    model: "deepseek-chat",
    embeddingModel: "deepseek-embedding",
  },
  moonshot: {
    id: "moonshot",
    name: "Moonshot",
    baseUrl: "https://api.moonshot.cn",
    model: "moonshot-v1-8k",
    embeddingModel: "moonshot-embedding-v1",
  },
  zhipu: {
    id: "zhipu",
    name: "智谱",
    baseUrl: "https://open.bigmodel.cn/api/paas/v4",
    model: "glm-4",
    embeddingModel: "embedding-3",
  },
  bailian: {
    id: "bailian",
    name: "百炼",
    baseUrl: "https://dashscope.aliyuncs.com/api/v1",
    model: "qwen-turbo",
    embeddingModel: "text-embedding-v2",
  },
};