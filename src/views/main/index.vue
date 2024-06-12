<template>
  <div class="space-y-14">
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
      <el-input placeholder="或者手动输入路径" v-model="mergePath" />
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
      <select-files v-model:target="mergePath" ref="getSelectPathList"></select-files>
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

async function doStartMerge() {
  const CONFIRM_NUM = 100
  if (!mergePath.value) {
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
  for (let path of selectPathList) {
    let mergeRes: DataResponse<string> = await invoke('merge_files', {
      path,
      exclude: getExcludeList.value.getExcludeList()
    })
    if (!mergeRes.success) {
      continue
    }
    mergedString.value += mergeRes.data
  }
  showMergedResult.value = true
}


const { run: startMergeRun, loading: startMergeLoading } = useRequest(doStartMerge, {
  manual: true,
  loadingDelay: 400,
  loadingKeep: 1000
})

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

onMounted(() => {
  listen('tauri://drop', (event: { payload: { paths: string[] } }) => {
    if (event.payload.paths.length > 1) {
      ElMessage.error('只能选择一个文件夹')
      return
    }
    mergePath.value = event.payload.paths[0]
  })
})

const getSelectPathList = ref()
const getExcludeList = ref()
const showMergedResult = ref<boolean>(false)
const showSelect = ref<boolean>(false)
const mergePath = ref<string>('')

async function selectMergeFolder() {
  mergePath.value = await selectFolder()
}

async function copyResult() {
  await writeText(mergedString.value)
  ElMessage.success(`已将结果复制到剪贴板，共计 ${mergedString.value.length} 个字符`)
}

const mergedString = ref<string>('')

watch(mergePath, async () => {
  console.log('改变了')
  let res: DataResponse<boolean> = await invoke('is_existing_directory', { path: mergePath.value })
  if (!res.success) {
    return
  }
  if (res.data === false) {
    return
  }
  showSelect.value = false
  showMergedResult.value = false
  await nextTick()
  showSelect.value = true
})
</script>
