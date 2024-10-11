<template>
  <el-button class="fixed bottom-20 right-20" circle @click="toggleDark">
    <el-icon v-if="config.theme === 'dark'">
      <Sunny />
    </el-icon>
    <el-icon v-if="config.theme === 'light'">
      <Moon />
    </el-icon>
  </el-button>
</template>
<script setup lang="ts">
import { Moon, Sunny } from '@element-plus/icons-vue'
import { useDark } from '@vueuse/core'
import { onMounted } from 'vue'
import { relaunch } from '@tauri-apps/plugin-process'
import { ElMessageBox } from 'element-plus'
import { useConfigStore } from '@/stores/config.ts'

let config = useConfigStore()

const isDark = useDark()

onMounted(async () => {})

const toggleDark = async () => {
  isDark.value = !isDark.value
  config.theme = isDark.value ? 'dark' : 'light'
  let b = await askRelaunch()
  if (!b) {
    return
  }
  await relaunch()
}

// 确认是否继续合并
async function askRelaunch(): Promise<boolean> {
  try {
    await ElMessageBox.confirm('重启软件后才会生效，是否立刻重启？', '警告', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
  } catch (e) {
    return false
  }
  return true
}
</script>
