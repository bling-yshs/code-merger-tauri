import { open } from '@tauri-apps/plugin-dialog'

export async function selectFolder(): Promise<string> {
  const path = await open({
    directory: true
  })
  if (path === null) {
    return ''
  }
  return path
}
