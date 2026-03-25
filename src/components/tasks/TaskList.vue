<template>
  <div class="task-list">
    <div class="list-header">
      <span>任务列表</span>
      <el-button type="primary" size="small" @click="showCreateDialog = true">
        <el-icon><Plus /></el-icon>
        新建
      </el-button>
    </div>

    <el-scrollbar class="list-scroll">
      <div v-if="tasksStore.loading" class="loading">
        <el-icon class="is-loading"><Loading /></el-icon>
      </div>

      <div v-else-if="tasksStore.tasks.length === 0" class="empty">
        暂无任务
      </div>

      <div v-else class="task-items">
        <TaskItem
          v-for="task in tasksStore.tasks"
          :key="task.id"
          :task="task"
        />
      </div>
    </el-scrollbar>

    <CreateTaskDialog
      v-model="showCreateDialog"
      @created="handleTaskCreated"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { Plus, Loading } from "@element-plus/icons-vue";
import { useTasksStore } from "@/stores/tasks";
import { useDocumentsStore } from "@/stores/documents";
import TaskItem from "./TaskItem.vue";
import CreateTaskDialog from "./CreateTaskDialog.vue";

const tasksStore = useTasksStore();
const documentsStore = useDocumentsStore();

const showCreateDialog = ref(false);

onMounted(() => {
  tasksStore.loadTasks();
  documentsStore.loadDocuments();
  tasksStore.setupListener();
});

function handleTaskCreated() {
  tasksStore.loadTasks();
}
</script>

<style scoped>
.task-list {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  font-weight: 600;
  border-bottom: 1px solid var(--el-border-color);
}

.list-scroll {
  flex: 1;
}

.loading,
.empty {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100px;
  color: var(--el-text-color-secondary);
}

.task-items {
  padding: 8px;
}
</style>