import { defineStore } from 'pinia'
import { computed, ComputedRef, Ref, ref, watch } from 'vue'
import { createStore } from '@tauri-apps/plugin-store'
import { useDark, useToggle } from '@vueuse/core'

export const useConfigStore = defineStore('config', () => {
  const excludeExts = ref<Array<string>>([])
  const excludeDirs = ref<Array<string>>([])
  const remindNum = ref(0)
  const enableGitignore = ref(false)
  const isDark = useDark()
  const theme = computed(() => {
    return isDark.value === true ? 'dark' : 'light'
  })
  
  // action
  const toggleTheme = useToggle(isDark)

  // 通用的 watch 和保存逻辑
  const watchAndSave = <T>(source: Ref<T> | ComputedRef<T>, key: string, options = {}) => {
    watch(
      source,
      async (value) => {
        const dbStore = await createStore('code-merger-tauri.bin')
        await dbStore.set(key, value)
        await dbStore.save()
      },
      options
    )
  }

  // 使用通用的 watch 函数
  watchAndSave(enableGitignore, 'enableGitignore')
  watchAndSave(remindNum, 'remindNum')
  watchAndSave(theme, 'theme')
  watchAndSave(excludeExts, 'excludeExts', { deep: true })
  watchAndSave(excludeDirs, 'excludeDirs', { deep: true })
  return { theme, excludeExts, excludeDirs, remindNum, enableGitignore, isDark, toggleTheme }
})

export const initConfigStore = async () => {
  const dbStore = await createStore('code-merger-tauri.bin')
  const configStore = useConfigStore()
  configStore.isDark = (await dbStore.get('theme')) === 'dark'
  configStore.excludeExts = (await dbStore.get('excludeExts')) || new Array<string>()
  configStore.excludeDirs = (await dbStore.get('excludeDirs')) || new Array<string>()
  configStore.remindNum = (await dbStore.get('remindNum')) || 100
  configStore.enableGitignore = (await dbStore.get('enableGitignore')) || false
}
