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
import { Store } from '@tauri-apps/plugin-store'

const isDark = useDark()
const store = new Store('code-merger-tauri.bin')
onMounted(async () => {
  isDark.value = (await store.get('isDark')) || false
})

const toggleDark = async () => {
  isDark.value = !isDark.value
  await store.set('isDark', isDark.value)
  await store.save()
}
</script>

<style scoped></style>
