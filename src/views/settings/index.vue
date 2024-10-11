<template>
  <div class="space-y-16">
    <exclude-extension />
    <exclude-path />
    <div class="flex items-center gap-10">
      <span>遵循 .gitignore 文件(开发中，无实际效果)</span>
      <el-switch v-model="config.enableGitignore" />
    </div>
  </div>
</template>
<script setup lang="ts">
import ExcludeExtension from '@/compoments/exclude-extension.vue'
import ExcludePath from '@/compoments/exclude-path.vue'
import { useConfigStore } from '@/stores/config.ts'

let config = useConfigStore()
</script>
