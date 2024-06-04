<template>
  <div class="space-y-4">
    <el-input v-model="inputValue" @keyup.enter="handleInput" placeholder="请输入需要排除的后缀名">
    </el-input>
    <div class="tags">
      <el-tag v-for="(tag, index) in excludeList" :key="index" closable @close="removeTag(index)">
        {{ tag }}
      </el-tag>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

const inputValue = ref('')
const excludeList = ref<Array<string>>(JSON.parse(localStorage.getItem('excludeList') || '[]'))

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

// 监控 excludeList 的变化，实时写入 localStorage
watch(
  excludeList,
  () => {
    localStorage.setItem('excludeList', JSON.stringify(excludeList.value))
  },
  { deep: true }
)

// 暴露 getExcludeList 方法给父组件
defineExpose({ getExcludeList })
</script>
