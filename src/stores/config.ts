import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import { createStore } from '@tauri-apps/plugin-store'

export const useConfigStore = defineStore('config', () => {
  const theme = ref<string>('')
  const excludeExts = ref<Array<string>>([])

  watch(
    theme,
    async (value) => {
      const dbStore = await createStore('code-merger-tauri.bin')
      await dbStore.set('theme', value)
      await dbStore.save()
    },
    {
      deep: true
    }
  )

  watch(
    excludeExts,
    async (value) => {
      const dbStore = await createStore('code-merger-tauri.bin')
      await dbStore.set('excludeExts', value)
      await dbStore.save()
    },
    {
      deep: true
    }
  )
  return { theme, excludeExts }
})

export const initConfigStore = async () => {
  const dbStore = await createStore('code-merger-tauri.bin')
  const configStore = useConfigStore()
  configStore.theme = (await dbStore.get('theme')) || 'light'
  configStore.excludeExts = (await dbStore.get('excludeExts')) || new Array<string>()
}
