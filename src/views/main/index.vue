<template>
  <div class="space-y-14">
    <div>
      <el-alert :closable="false" title="您可以将文件夹拖入任意位置来开始" type="success" />
    </div>
    <div class="flex gap-10">
      <div>
        <el-button @click="selectMergeFolder">
          <el-icon>
            <folder-opened></folder-opened>
          </el-icon>
          <span>选择文件夹</span>
        </el-button>
      </div>
      <input-merge-path />
      <div>
        <el-button type="primary" @click="copyResult">
          <span>复制结果</span>
        </el-button>
      </div>
    </div>

    <!--选择框-->
    <div v-if="global.showSelectTree">
      <select-files />
    </div>

    <div v-if="global.showMergeResult">
      <el-input
        v-model="global.mergeResult"
        type="textarea"
        autosize
        placeholder="合并结果"
        resize="none"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { FolderOpened } from '@element-plus/icons-vue'
import { writeText } from '@tauri-apps/plugin-clipboard-manager'
import SelectFiles from '@/compoments/select-files.vue'
import { selectFolder } from '@/utils/path-utils'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage } from 'element-plus'
import { listen } from '@tauri-apps/api/event'
import { useGlobalStore } from '@/stores/global.ts'
import InputMergePath from '@/compoments/input-merge-path.vue'

const global = useGlobalStore()

// 监听拖拽事件
onMounted(() => {
  listen('tauri://drag-drop', (event: { payload: { paths: string[] } }) => {
    console.log('触发了拖拽事件: ', event)
    if (event.payload.paths.length > 1) {
      ElMessage.error('只能选择一个文件夹')
      return
    }
    global.pathToMerge = event.payload.paths[0]
  })
})

// 选择文件夹
async function selectMergeFolder() {
  global.pathToMerge = await selectFolder()
}

// 复制结果到剪贴板
async function copyResult() {
  await writeText(global.mergeResult)
  const tokens = await countTokens(global.mergeResult)
  ElMessage.success(`已将结果复制到剪贴板，共计 ${tokens} 个 Tokens`)
}

async function countTokens(content: string): Promise<number> {
  const res: DataResponse<number> = await invoke('count_tokens', {
    content
  })
  if (!res.success) {
    ElMessage.error('统计 Tokens 数量失败')
    return 0
  }
  return res.data
}
</script>
