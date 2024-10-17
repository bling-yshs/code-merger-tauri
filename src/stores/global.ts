import { defineStore } from 'pinia'
import { nextTick, ref, watch } from 'vue'
import { ElMessageBox, ElTree } from 'element-plus'
import type Node from 'element-plus/es/components/tree/src/model/node'
import { invoke } from '@tauri-apps/api/core'

export const useGlobalStore = defineStore('global', () => {
  const pathToMerge = ref('')

  watch(pathToMerge, async () => {
    const res: DataResponse<boolean> = await invoke('is_existing_directory', {
      path: pathToMerge.value
    })
    if (!res.success) {
      return
    }
    if (res.data === false) {
      return
    }
    showMergeResult.value = false
    // 这里需要重新渲染才能更新树的根节点
    showSelectTree.value = false
    await nextTick()
    showSelectTree.value = true
  })

  const mergeResult = ref('')
  const showMergeResult = ref(false)
  const treeRef = ref<InstanceType<typeof ElTree>>()
  const showSelectTree = ref(false)
  const inputBoxValue = ref('')

  const getSelectPathList = (): Array<string> => {
    if (!treeRef.value) {
      return []
    }
    const selectPaths: Array<string> = treeRef.value.getCheckedNodes().map((node) => node.path)

    return selectPaths
  }

  const getNoSelectPathList = (): Array<string> => {
    const getAllNodeList = (node: Node): Array<Node> => {
      const result = new Array<Node>()
      result.push(node)
      for (const each of node.childNodes) {
        result.push(...getAllNodeList(each))
      }
      return result
    }

    const node = treeRef.value?.getNode(pathToMerge.value)
    if (!node) {
      return []
    }
    const allNodeList = getAllNodeList(node)

    return allNodeList
      .filter((each) => !each.indeterminate && !each.checked)
      .map((each) => each.data.path)
  }

  const confirmMerge = async (num: number): Promise<boolean> => {
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

  return {
    pathToMerge,
    mergeResult,
    showMergeResult,
    treeRef,
    getSelectPathList,
    inputBoxValue,
    getNoSelectPathList,
    showSelectTree,
    confirmMerge
  }
})
