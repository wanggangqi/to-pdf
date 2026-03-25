<template>
  <div class="home-view">
    <h1>DocTranslate</h1>
    <p>文档翻译与智能问答</p>
    <div class="actions">
      <el-button type="primary" @click="$router.push('/settings')">
        设置
      </el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
// Home view placeholder
</script>

<style scoped>
.home-view {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
}

.home-view h1 {
  font-size: 2rem;
  margin-bottom: 1rem;
}

.home-view p {
  color: #666;
  margin-bottom: 2rem;
}

.actions {
  display: flex;
  gap: 1rem;
}
</style>