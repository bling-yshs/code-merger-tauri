<template>
  <div class="space-y-16">
    <exclude-extension />
    <exclude-path />
    <div class="flex items-center gap-10">
      <div class="flex items-center gap-5">
        <span class="text-14 dark:color-gray-4">确认阈值</span>
        <el-tooltip content="当合并的文件数量超过这个数，则合并时会有二次确认提示框，0表示无限制">
          <el-icon>
            <QuestionFilled class="color-gray-4" />
          </el-icon>
        </el-tooltip>
      </div>
      <el-input-number
        controls-position="right"
        v-model="config.remindNum"
        style="width: 100px"
      ></el-input-number>
    </div>
    <div class=" flex items-center gap-10">
      <span class="text-14 dark:color-gray-4">遵循 .gitignore 文件</span>
      <el-switch size="small" v-model="config.enableGitignore" />
    </div>
  </div>
</template>
<script setup lang="ts">
import ExcludeExtension from '@/compoments/exclude-extension.vue'
import ExcludePath from '@/compoments/exclude-path.vue'
import { useConfigStore } from '@/stores/config.ts'
import { QuestionFilled } from '@element-plus/icons-vue'

const config = useConfigStore()
</script>
