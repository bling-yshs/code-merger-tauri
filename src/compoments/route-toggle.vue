<template>
  <el-tooltip :content='target.name' placement="left">
    <el-button class="fixed bottom-60 right-20" @click="toggle" circle>
      <el-icon v-if="target.name === '设置'">
        <setting />
      </el-icon>
      <el-icon v-else>
        <house />
      </el-icon>
    </el-button>
  </el-tooltip>
</template>
<script setup lang="ts">
import router from '@/router'
import { Setting } from '@element-plus/icons-vue'
import { ref } from 'vue'
import { House } from '@element-plus/icons-vue'

interface Target {
  name: string
  path: string
}

let target = ref<Target>({ name: '设置', path: '/settings' })

function toggle() {
  router.push(target.value.path)
  target.value =
    target.value.name === '设置' ? { name: '主页', path: '/' } : { name: '设置', path: '/settings' }
}
</script>
