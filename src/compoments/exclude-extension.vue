<template>
  <div class="space-y-4">
    <el-input v-model="inputValue" @keyup.enter="handleInput" placeholder="请输入需要排除的后缀名">
    </el-input>
    <div class="flex gap-4">
      <el-tag v-for="(tag, index) in excludeList" :key="index" closable @close="removeTag(index)">
        {{ tag }}
      </el-tag>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useLocalStorage } from '@vueuse/core'

const inputValue = ref('')

const handleInput = () => {
  if (inputValue.value.trim() !== '') {
    excludeList.value.push(inputValue.value)
    inputValue.value = ''
  }
}

const removeTag = (index: number) => {
  excludeList.value.splice(index, 1)
}

const getExcludeList = () => {
  return excludeList.value
}

const excludeList = useLocalStorage('excludeList', Array<String>)

// 暴露 getExcludeList 方法给父组件
defineExpose({ getExcludeList })
</script>
