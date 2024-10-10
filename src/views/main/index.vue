<template>
  <div class="space-y-14">
    <el-button @click="test">test</el-button>
    <div>
      <el-alert :closable="false" title="您可以将文件夹拖入任意位置来开始" type="success" />
    </div>
    <div>
      <exclude-extension ref="getExcludeList" />
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
      <el-input placeholder="或者手动输入路径" v-model="needMergedPath" />
      <div>
        <el-button type="primary" @click="startMergeRun" :loading="startMergeLoading">
          <span>开始合并</span>
        </el-button>
      </div>
      <div>
        <el-button type="primary" @click="copyResult">
          <span>复制结果</span>
        </el-button>
      </div>
    </div>

    <!--选择框-->
    <div v-if="showSelect">
      <select-files v-model:target="needMergedPath" ref="getSelectPathList"></select-files>
    </div>

    <div v-if="showMergedResult">
      <el-input
        v-model="mergedString"
        type="textarea"
        autosize
        placeholder="合并结果"
        resize="none"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { nextTick, onMounted, ref, watch } from 'vue'
import { FolderOpened } from '@element-plus/icons-vue'
import { writeText } from '@tauri-apps/plugin-clipboard-manager'
import SelectFiles from '@/compoments/select-files.vue'
import { selectFolder } from '@/utils/path-utils'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage, ElMessageBox } from 'element-plus'
import ExcludeExtension from '@/compoments/exclude-extension.vue'
import { listen } from '@tauri-apps/api/event'
import { useRequest } from 'vue-request'
import MergeFilesRequest from '@/interface/merge-files-request.ts'

const needMergedPath = ref<string>('')

async function test() {
  console.log('test')
  getSelectPathList.value.printAllNodes()
}

// 监听拖拽事件
onMounted(() => {
  listen('tauri://drag-drop', (event: { payload: { paths: string[] } }) => {
    console.log('触发了拖拽事件: ', event)
    if (event.payload.paths.length > 1) {
      ElMessage.error('只能选择一个文件夹')
      return
    }
    needMergedPath.value = event.payload.paths[0]
  })
})

// 选择文件夹
async function selectMergeFolder() {
  needMergedPath.value = await selectFolder()
}

// 点击开始合并的逻辑
async function doStartMerge() {
  const CONFIRM_NUM = 100
  if (!needMergedPath.value) {
    ElMessage.error('请先选择文件夹')
    return
  }
  showMergedResult.value = false
  mergedString.value = ''
  let selectPathList: Array<string> = getSelectPathList.value.getSelectPathList()
  let res: DataResponse<boolean> = await invoke('are_files_less_than', {
    paths: selectPathList,
    num: CONFIRM_NUM
  })
  if (!res.success) {
    return
  }
  if (res.data === false) {
    if (!(await confirmMerge(CONFIRM_NUM))) {
      return
    }
  }
  const rootPath = needMergedPath.value
  const excludeExts = getExcludeList.value.getExcludeList()
  const excludePaths = new Array<string>()
  // todo 这里换乘循环获取选中的节点然后发送到rust
  let request = new MergeFilesRequest(rootPath, excludeExts, excludePaths)
  let mergeRes: DataResponse<string> = await invoke('merge_files', {
    request: request
  })
  if (!mergeRes.success) {
    ElMessage.error(mergeRes.message)
    return
  }
  mergedString.value = mergeRes.data
  showMergedResult.value = true
}

// loadingKeep，防止闪烁
const { run: startMergeRun, loading: startMergeLoading } = useRequest(doStartMerge, {
  manual: true,
  loadingDelay: 400,
  loadingKeep: 1000
})

// 确认是否继续合并
async function confirmMerge(num: number): Promise<boolean> {
  try {
    await ElMessageBox.confirm(`当前文件夹的文件数量大于 ${num} ，确定要继续吗?`, '警告', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
  } catch (e) {
    return false
  }
  return true
}

const getSelectPathList = ref()
const getExcludeList = ref()
const showMergedResult = ref<boolean>(false)
const showSelect = ref<boolean>(false)
const mergedString = ref<string>('')

// 监听 needMergedPath 的变化，实时显示选择树
watch(needMergedPath, async () => {
  let res: DataResponse<boolean> = await invoke('is_existing_directory', {
    path: needMergedPath.value
  })
  if (!res.success) {
    return
  }
  if (res.data === false) {
    return
  }
  showMergedResult.value = false
  // 这里需要重新渲染才能更新树的根节点
  showSelect.value = false
  await nextTick()
  showSelect.value = true
})

// 复制结果到剪贴板
async function copyResult() {
  await writeText(mergedString.value)
  let tokens = await countTokens(mergedString.value)
  ElMessage.success(`已将结果复制到剪贴板，共计 ${tokens} 个 Tokens`)
}

async function countTokens(content: string): Promise<number> {
  let res: DataResponse<number> = await invoke('count_tokens', {
    content
  })
  if (!res.success) {
    ElMessage.error('统计 Tokens 数量失败')
    return 0
  }
  return res.data
}
</script>
