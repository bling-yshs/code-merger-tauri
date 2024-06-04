<template>
  <div :class="{ 'white-bg': whiteBg, global: true }">
    <el-scrollbar>
      <global-view />
    </el-scrollbar>
  </div>
</template>

<script setup lang="ts">
import GlobalView from '@/views/global-view.vue'
import { onMounted, ref } from 'vue'
import { version } from '@tauri-apps/plugin-os'

const whiteBg = ref<boolean>(true)

onMounted(async () => {
  let windowsVersion = await version()
  if (windowsVersion.split('.')[2] >= '22000') {
    whiteBg.value = false
  }
})
</script>

<style scoped>
.global {
  height: 100%;
}

.white-bg {
  background-color: #fff;
}
</style>
