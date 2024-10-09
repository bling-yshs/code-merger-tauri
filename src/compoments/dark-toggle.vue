<template>
  <el-button class="fixed bottom-20 right-20" circle @click="toggleDark">
    <el-icon v-if="!isDark">
      <Sunny />
    </el-icon>
    <el-icon v-if="isDark">
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
import { createStore } from '@tauri-apps/plugin-store'

const isDark = useDark()

onMounted(async () => {
  const store = await createStore('code-merger-tauri.bin')
  isDark.value = (await store.get<boolean>('isDark')) || false
})

const toggleDark = async () => {
  isDark.value = !isDark.value
  const store = await createStore('code-merger-tauri.bin')
  await store.set('isDark', isDark.value)
  await store.save()
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

<style scoped></style>
