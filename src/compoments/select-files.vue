<template>
  <el-tree
    ref="treeRef"
    :props="props"
    node-key="path"
    :default-checked-keys="[global.pathToMerge]"
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
import GetSubFilesRequest from '@/interface/get-sub-files-request.ts'
import { useGlobalStore } from '@/stores/global.ts'
import { onMounted, ref } from 'vue'

const global = useGlobalStore()

const treeRef = ref<InstanceType<typeof ElTree>>()

onMounted(() => {
  global.treeRef = treeRef.value
})

interface Tree {
  name: string
  path: string
  relativePath: string
  leaf?: boolean
}

const props = {
  label: 'name',
  children: 'zones',
  isLeaf: 'leaf'
}

async function loadNode(node: Node, resolve: (data: Tree[]) => void) {
  if (node.level === 0) {
    return resolve([
      { name: await basename(global.pathToMerge), path: global.pathToMerge, relativePath: '' }
    ])
  }
  const request = new GetSubFilesRequest(global.pathToMerge, node.data.path)
  const res: DataResponse<Array<MyFile>> = await invoke('get_sub_files', {
    request: request
  })
  if (!res.success) {
    return
  }
  const data: Tree[] = []
  for (const item of res.data) {
    data.push({
      name: await basename(item.path),
      path: item.path,
      relativePath: item.relativePath,
      leaf: item.isFolderEmpty || !item.isFolder
    })
  }
  return resolve(data)
}
</script>
