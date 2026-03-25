import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { Task } from "@/types";

interface TaskProgress {
  taskId: string;
  status: string;
  progress: number;
}

export const useTasksStore = defineStore("tasks", {
  state: () => ({
    tasks: [] as Task[],
    loading: false,
    progressMap: new Map<string, number>(),
  }),

  getters: {
    getTaskById: (state) => (id: string) =>
      state.tasks.find((t) => t.id === id),
    pendingTasks: (state) =>
      state.tasks.filter((t) => t.status === "pending"),
    processingTasks: (state) =>
      state.tasks.filter((t) => t.status === "processing"),
    completedTasks: (state) =>
      state.tasks.filter((t) => t.status === "completed"),
    failedTasks: (state) =>
      state.tasks.filter((t) => t.status === "failed"),
  },

  actions: {
    async loadTasks() {
      this.loading = true;
      try {
        this.tasks = await invoke<Task[]>("list_tasks");
      } finally {
        this.loading = false;
      }
    },

    async createTask(documentId: string) {
      const task = await invoke<Task>("create_task", { documentId });
      await this.loadTasks();
      return task;
    },

    async deleteTask(id: string) {
      await invoke("delete_task", { id });
      await this.loadTasks();
    },

    async cancelTask(id: string) {
      await invoke("cancel_task", { id });
      await this.loadTasks();
    },

    async openTaskOutput(id: string) {
      await invoke("open_task_output", { id });
    },

    setupListener() {
      listen<TaskProgress>("task-progress", (event) => {
        const { taskId, status, progress } = event.payload;
        this.progressMap.set(taskId, progress);

        if (status === "completed" || status === "failed") {
          // 刷新任务列表
          this.loadTasks();
          // 清除进度缓存
          this.progressMap.delete(taskId);
        }
      });
    },

    getProgress(taskId: string): number {
      const task = this.tasks.find((t) => t.id === taskId);
      return this.progressMap.get(taskId) ?? task?.progress ?? 0;
    },
  },
});