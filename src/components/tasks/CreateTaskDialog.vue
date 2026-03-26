<template>
  <el-dialog
    v-model="visible"
    title="创建翻译任务"
    width="500px"
    :close-on-click-modal="false"
  >
    <el-form :model="form" label-width="80px">
      <el-form-item label="选择文档" required>
        <el-select
          v-model="form.documentId"
          placeholder="请选择要翻译的文档"
          style="width: 100%"
          :loading="documentsStore.loading"
        >
          <el-option
            v-for="doc in availableDocuments"
            :key="doc.id"
            :label="doc.name"
            :value="doc.id"
          >
            <div class="doc-option">
              <span class="doc-name">{{ doc.name }}</span>
              <el-tag size="small" :type="doc.type === 'word' ? 'primary' : 'info'">
                {{ doc.type === 'word' ? 'Word' : 'PDF' }}
              </el-tag>
            </div>
          </el-option>
        </el-select>
      </el-form-item>

      <el-form-item label="说明">
        <div class="info-text">
          任务将把选中的中文文档翻译为双语PDF（段落对照格式）。
          翻译过程中会使用AI进行文本翻译，处理时间取决于文档长度。
        </div>
      </el-form-item>
    </el-form>

    <template #footer>
      <el-button @click="visible = false">取消</el-button>
      <el-button
        type="primary"
        :loading="creating"
        :disabled="!form.documentId"
        @click="handleCreate"
      >
        创建任务
      </el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from "vue";
import { useDocumentsStore } from "@/stores/documents";
import { useTasksStore } from "@/stores/tasks";
import { ElMessage } from "element-plus";

const visible = defineModel<boolean>();

const emit = defineEmits<{
  created: [];
}>();

const documentsStore = useDocumentsStore();
const tasksStore = useTasksStore();

const form = ref({
  documentId: "",
});

const creating = ref(false);

// 可选择的文档：排除正在处理中的任务
const availableDocuments = computed(() => {
  const processingDocIds = new Set(
    tasksStore.processingTasks.map((t) => t.documentId)
  );
  return documentsStore.documents.filter(
    (doc) => !processingDocIds.has(doc.id)
  );
});

// 打开对话框时加载文档列表
watch(visible, (val) => {
  if (val) {
    documentsStore.loadDocuments();
    form.value.documentId = "";
  }
});

async function handleCreate() {
  if (!form.value.documentId) {
    return;
  }

  creating.value = true;
  try {
    await tasksStore.createTask(form.value.documentId);
    ElMessage.success("任务创建成功");
    visible.value = false;
    emit("created");
  } catch (error) {
    ElMessage.error("创建任务失败: " + (error as Error).message);
  } finally {
    creating.value = false;
  }
}
</script>

<style scoped>
.doc-option {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
}

.doc-name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.info-text {
  font-size: 13px;
  color: var(--el-text-color-secondary);
  line-height: 1.6;
}
</style>