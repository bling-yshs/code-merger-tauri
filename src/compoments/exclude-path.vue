<template>
  <div class="space-y-4">
    <el-input
      v-model="inputBoxValue"
      @keyup.enter="handleEnter"
      placeholder="请输入需要全局固定排除的路径，如: build，输入完请按回车确认"
    >
    </el-input>
    <div class="flex gap-4">
      <el-tag v-for="(tag, index) in configStore.excludeDirs" :key="index" closable @close="removeTag(index)">
        {{ tag }}
      </el-tag>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useConfigStore } from '@/stores/config.ts'

const configStore = useConfigStore()
let inputBoxValue = ref('')

async function handleEnter() {
  if (inputBoxValue.value.trim() !== '') {
    // 添加到 excludeList
    configStore.excludeDirs.push(inputBoxValue.value.trim())
    // 清空输入框
    inputBoxValue.value = ''
  }
}

function removeTag(index: number) {
  configStore.excludeDirs.splice(index, 1)
}

</script>
