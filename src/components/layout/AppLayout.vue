<template>
  <div class="app-layout">
    <el-container>
      <el-aside width="200px">
        <el-menu router>
          <el-menu-item index="/">
            <el-icon><HomeFilled /></el-icon>
            <span>首页</span>
          </el-menu-item>
          <el-menu-item index="/settings">
            <el-icon><Setting /></el-icon>
            <span>设置</span>
          </el-menu-item>
        </el-menu>
      </el-aside>
      <el-main>
        <slot />
      </el-main>
    </el-container>
  </div>
</template>

<script setup lang="ts">
import { HomeFilled, Setting } from "@element-plus/icons-vue";
</script>

<style scoped>
.app-layout {
  height: 100%;
}

.el-container {
  height: 100%;
}

.el-aside {
  background-color: #f5f7fa;
  border-right: 1px solid #e4e7ed;
}

.el-menu {
  border-right: none;
}
</style>