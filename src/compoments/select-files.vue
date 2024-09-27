<template>
  <el-tree
    ref="treeRef"
    :props="props"
    node-key="path"
    :default-checked-keys="[target]"
    :load="loadNode"
    lazy
    show-checkbox
  />
</template>

<script lang="ts" setup>
import type Node from 'element-plus/es/components/tree/src/model/node'
import { basename } from '@tauri-apps/api/path'
import { invoke } from '@tauri-apps/api/core'
import type MyFile from '@/interface/my-file'
import { ElTree } from 'element-plus'
import { ref } from 'vue'

const treeRef = ref<InstanceType<typeof ElTree>>()

interface Tree {
  name: string
  path: string
  leaf?: boolean
}

const props = {
  label: 'name',
  children: 'zones',
  isLeaf: 'leaf'
}

async function loadNode(node: Node, resolve: (data: Tree[]) => void) {
  if (node.level === 0) {
    return resolve([{ name: await basename(target.value), path: target.value }])
  }
  let res: DataResponse<Array<MyFile>> = await invoke('get_sub_files', { path: node.data.path })
  if (!res.success) {
    return
  }
  const data: Tree[] = []
  for (let item of res.data) {
    data.push({
      name: await basename(item.path),
      path: item.path,
      leaf: item.isFolderEmpty || !item.isFolder
    })
  }
  return resolve(data)
}

let target = defineModel<string>('target', {
  required: true
})

function getSelectPathList(): Array<string> {
  if (!treeRef.value) {
    return []
  }
  const checkedNodes = treeRef.value.getCheckedNodes(true)
  let map = checkedNodes.map((node) => node.path)
  return map
}

defineExpose({
  getSelectPathList
})
</script>
