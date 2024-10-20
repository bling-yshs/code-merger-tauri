<template>
  <el-input placeholder="或者手动输入路径" v-model="global.pathToMerge" />
  <div>
    <el-button type="primary" @click="merge" :loading="isMerging">
      <span>开始合并</span>
    </el-button>
  </div>
</template>
<script setup lang="ts">
import { useGlobalStore } from '@/stores/global.ts'
import { useRequest } from 'vue-request'
import { ElMessage } from 'element-plus'
import { invoke } from '@tauri-apps/api/core'
import MergeFilesRequest from '@/interface/merge-files-request.ts'
import { useConfigStore } from '@/stores/config.ts'
import AreFilesLessThanRequest from '@/interface/are-files-less-than-request.ts'

const global = useGlobalStore()
const config = useConfigStore()

// loadingKeep，防止闪烁
const { run: merge, loading: isMerging } = useRequest(doMerge, {
  manual: true,
  loadingDelay: 400,
  loadingKeep: 1000
})

// 点击开始合并的逻辑
async function doMerge() {
  if (!global.pathToMerge) {
    ElMessage.error('请先选择文件夹')
    return
  }
  global.showMergeResult = false
  global.mergeResult = ''
  const rootPath = global.pathToMerge
  const excludeExts = config.excludeExts
  const excludePaths = Array.from(
    new Set([...global.getNoSelectPathList(), ...config.excludePaths])
  )
  const enableGitignore = config.enableGitignore
  const remindNum = config.remindNum
  let areFilesLessThanRequest = new AreFilesLessThanRequest(
    rootPath,
    remindNum,
    excludeExts,
    excludePaths,
    enableGitignore
  )
  // 如果 remindNum 不为 0，且文件数量大于 remindNum，询问是否继续
  if (remindNum !== 0) {
    let res: DataResponse<boolean> = await invoke('are_files_less_than', {
      request: areFilesLessThanRequest
    })
    if (!res.success) {
      return
    }
    if (res.data === false) {
      if (!(await global.confirmMerge(config.remindNum))) {
        return
      }
    }
  }
  let request = new MergeFilesRequest(rootPath, excludeExts, excludePaths, enableGitignore)
  let mergeRes: DataResponse<string> = await invoke('merge_files', {
    request: request
  })
  if (!mergeRes.success) {
    ElMessage.error(mergeRes.message)
    return
  }
  global.mergeResult = mergeRes.data
  global.showMergeResult = true
}
</script>
