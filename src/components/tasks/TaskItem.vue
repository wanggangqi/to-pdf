<template>
  <div class="task-item" :class="statusClass">
    <div class="task-header">
      <el-icon class="task-icon" :class="statusClass">
        <component :is="statusIcon" />
      </el-icon>
      <div class="task-name">{{ documentName }}</div>
      <el-tag size="small" :type="tagType">
        {{ statusText }}
      </el-tag>
    </div>

    <div class="task-progress" v-if="task.status === 'processing'">
      <el-progress
        :percentage="progressDetail.progress"
        :stroke-width="6"
        :show-text="false"
      />
      <span class="progress-text">
        <template v-if="progressDetail.phase === 'translation' && progressDetail.completed && progressDetail.total">
          翻译 {{ progressDetail.completed }}/{{ progressDetail.total }}
        </template>
        <template v-else>
          {{ progressDetail.progress }}%
        </template>
      </span>
    </div>

    <div class="task-meta">
      <span>{{ formatTime(task.createdAt) }}</span>
      <span v-if="task.completedAt" class="completed-time">
        完成于 {{ formatTime(task.completedAt) }}
      </span>
    </div>

    <div v-if="task.error" class="task-error">
      <el-icon><WarningFilled /></el-icon>
      {{ task.error }}
    </div>

    <div class="task-actions">
      <el-button
        v-if="task.status === 'completed' && task.outputPath"
        type="primary"
        size="small"
        link
        @click="handleOpenOutput"
      >
        打开文件
      </el-button>
      <el-button
        v-if="task.status === 'processing'"
        type="warning"
        size="small"
        link
        @click="handleCancel"
      >
        取消
      </el-button>
      <el-button
        type="danger"
        size="small"
        link
        @click="handleDelete"
      >
        删除
      </el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, onMounted, onUnmounted } from "vue";
import {
  Clock,
  Loading,
  CircleCheck,
  CircleClose,
  WarningFilled,
} from "@element-plus/icons-vue";
import type { Task } from "@/types";
import { useTasksStore } from "@/stores/tasks";
import { useDocumentsStore } from "@/stores/documents";

const props = defineProps<{
  task: Task;
}>();

const tasksStore = useTasksStore();
const documentsStore = useDocumentsStore();

// 实时进度
const liveProgress = ref(0);
let progressInterval: ReturnType<typeof setInterval> | null = null;

const statusIcon = computed(() => {
  switch (props.task.status) {
    case "pending":
      return Clock;
    case "processing":
      return Loading;
    case "completed":
      return CircleCheck;
    case "failed":
      return CircleClose;
    default:
      return Clock;
  }
});

const statusClass = computed(() => props.task.status);

const statusText = computed(() => {
  switch (props.task.status) {
    case "pending":
      return "等待中";
    case "processing":
      return "处理中";
    case "completed":
      return "已完成";
    case "failed":
      return "失败";
    default:
      return "未知";
  }
});

const tagType = computed(() => {
  switch (props.task.status) {
    case "pending":
      return "info";
    case "processing":
      return "warning";
    case "completed":
      return "success";
    case "failed":
      return "danger";
    default:
      return "info";
  }
});

const progressDetail = computed(() => {
  if (props.task.status === "processing") {
    return tasksStore.getProgress(props.task.id);
  }
  return { progress: props.task.progress };
});

const documentName = computed(() => {
  const doc = documentsStore.documents.find(
    (d) => d.id === props.task.documentId
  );
  return doc?.name ?? "未知文档";
});

function formatTime(isoString: string): string {
  const date = new Date(isoString);
  return date.toLocaleString("zh-CN", {
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  });
}

async function handleOpenOutput() {
  await tasksStore.openTaskOutput(props.task.id);
}

async function handleCancel() {
  await tasksStore.cancelTask(props.task.id);
}

async function handleDelete() {
  await tasksStore.deleteTask(props.task.id);
}

// 处理中状态时更新进度
onMounted(() => {
  if (props.task.status === "processing") {
    progressInterval = setInterval(() => {
      // 强制响应式更新
      liveProgress.value = tasksStore.getProgress(props.task.id).progress;
    }, 500);
  }
});

onUnmounted(() => {
  if (progressInterval) {
    clearInterval(progressInterval);
  }
});
</script>

<style scoped>
.task-item {
  padding: 12px;
  border-radius: 8px;
  background: var(--el-fill-color-blank);
  border: 1px solid var(--el-border-color-light);
  margin-bottom: 8px;
  transition: all 0.2s;
}

.task-item:hover {
  border-color: var(--el-border-color);
}

.task-item.processing {
  border-color: var(--el-color-warning-light-3);
}

.task-item.completed {
  border-color: var(--el-color-success-light-3);
}

.task-item.failed {
  border-color: var(--el-color-danger-light-3);
}

.task-header {
  display: flex;
  align-items: center;
  gap: 8px;
}

.task-icon {
  font-size: 18px;
}

.task-icon.pending {
  color: var(--el-text-color-secondary);
}

.task-icon.processing {
  color: var(--el-color-warning);
  animation: spin 1s linear infinite;
}

.task-icon.completed {
  color: var(--el-color-success);
}

.task-icon.failed {
  color: var(--el-color-danger);
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.task-name {
  flex: 1;
  font-size: 14px;
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.task-progress {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 8px;
}

.task-progress .el-progress {
  flex: 1;
}

.progress-text {
  font-size: 12px;
  color: var(--el-text-color-secondary);
  min-width: 40px;
  text-align: right;
}

.task-meta {
  display: flex;
  gap: 12px;
  margin-top: 8px;
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.completed-time {
  color: var(--el-color-success);
}

.task-error {
  display: flex;
  align-items: center;
  gap: 4px;
  margin-top: 8px;
  font-size: 12px;
  color: var(--el-color-danger);
}

.task-actions {
  display: flex;
  gap: 8px;
  margin-top: 8px;
  justify-content: flex-end;
}
</style>