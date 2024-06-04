import { open } from '@tauri-apps/plugin-dialog'

export async function selectFolder(): Promise<string> {
  const path = await open({
    directory: true
  })
  if (path === null) {
    return ''
  }
  // 如果是数组
  if (Array.isArray(path)) {
    return ''
  }
  return path
}
