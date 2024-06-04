<template>
  <div class="space-y-14">
    <div>
      <el-alert :closable="false" title="您可以将文件夹拖入任意位置来开始" type="success" />
    </div>
    <div>
      <exclude-extension ref="getExcludeList" />
    </div>
    <!--<div>-->
    <!--  <el-button-->
    <!--    @click="-->
    <!--      () => {-->
    <!--        $router.push('/test')-->
    <!--      }-->
    <!--    "-->
    <!--  >-->
    <!--    <span>切换到 test</span>-->
    <!--  </el-button>-->
    <!--</div>-->
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
        <el-button type="primary" @click="startMerge">
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
import { onMounted, ref } from 'vue'
import { FolderOpened } from '@element-plus/icons-vue'
import { writeText } from '@tauri-apps/plugin-clipboard-manager'
import SelectFiles from '@/compoments/select-files.vue'
import { selectFolder } from '@/utils/path-utils'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage } from 'element-plus'
import ExcludeExtension from '@/compoments/exclude-extension.vue'
import { listen } from '@tauri-apps/api/event'

onMounted(() => {
  listen('tauri://drop', (event: { payload: { paths: string[] } }) => {
    console.log('触发了 drop')
    console.log(event)
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
  showSelect.value = false
  showMergedResult.value = false
  mergePath.value = await selectFolder()
  if (!mergePath.value) {
    return
  }
  showSelect.value = true
}

async function copyResult() {
  await writeText(mergedString.value)
  ElMessage.success(`已将结果复制到剪贴板，共计 ${mergedString.value.length} 个字符`)
}

const mergedString = ref<string>('')

async function startMerge() {
  if (!mergePath.value) {
    ElMessage.error('请先选择文件夹')
    return
  }
  mergedString.value = ''
  let selectPathList: Array<string> = getSelectPathList.value.getSelectPathList()
  for (let path of selectPathList) {
    let res: DataResponse<string> = await invoke('merge_files', {
      path,
      exclude: getExcludeList.value.getExcludeList()
    })
    if (!res.success) {
      continue
    }
    mergedString.value += res.data
  }
  showMergedResult.value = true
}
</script>
