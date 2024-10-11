import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import { createStore } from '@tauri-apps/plugin-store'

export const useConfigStore = defineStore('config', () => {
  const theme = ref<string>('')
  const excludeExts = ref<Array<string>>([])
  const excludePaths = ref<Array<string>>([])
  const remindNum = ref<number>(0)
  const enableGitignore = ref<boolean>(false)

  watch(enableGitignore, async (value) => {
    const dbStore = await createStore('code-merger-tauri.bin')
    await dbStore.set('enableGitignore', value)
    await dbStore.save()
  })

  watch(remindNum, async (value) => {
    const dbStore = await createStore('code-merger-tauri.bin')
    await dbStore.set('remindNum', value)
    await dbStore.save()
  })

  watch(theme, async (value) => {
    const dbStore = await createStore('code-merger-tauri.bin')
    await dbStore.set('theme', value)
    await dbStore.save()
  })

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

  watch(
    excludePaths,
    async (value) => {
      const dbStore = await createStore('code-merger-tauri.bin')
      await dbStore.set('excludePaths', value)
      await dbStore.save()
    },
    {
      deep: true
    }
  )
  return { theme, excludeExts, excludePaths, remindNum, enableGitignore }
})

export const initConfigStore = async () => {
  const dbStore = await createStore('code-merger-tauri.bin')
  const configStore = useConfigStore()
  configStore.theme = (await dbStore.get('theme')) || 'light'
  configStore.excludeExts = (await dbStore.get('excludeExts')) || new Array<string>()
  configStore.excludePaths = (await dbStore.get('excludePaths')) || new Array<string>()
  configStore.remindNum = (await dbStore.get('remindNum')) || 0
  configStore.enableGitignore = (await dbStore.get('enableGitignore')) || false
}
