<template>
  <el-tree
    ref="treeRef"
    :props="props"
    node-key="path"
    :default-checked-keys="[rootPath]"
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
import GetSubFilesRequest from '@/interface/get-sub-files-request.ts'

const treeRef = ref<InstanceType<typeof ElTree>>()

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
  console.log(node)
  if (node.level === 0) {
    return resolve([
      { name: await basename(rootPath.value), path: rootPath.value, relativePath: '' }
    ])
  }
  let request = new GetSubFilesRequest(rootPath.value, node.data.path)
  console.log(request)
  let res: DataResponse<Array<MyFile>> = await invoke('get_sub_files', {
    request: request
  })
  if (!res.success) {
    return
  }
  const data: Tree[] = []
  for (let item of res.data) {
    data.push({
      name: await basename(item.path),
      path: item.path,
      relativePath: item.relativePath,
      leaf: item.isFolderEmpty || !item.isFolder
    })
  }
  return resolve(data)
}

let rootPath = defineModel<string>('rootPath', {
  required: true
})

function getSelectPathList(): Array<string> {
  if (!treeRef.value) {
    return []
  }
  const selectPaths = treeRef.value.getCheckedNodes().map((node) => node.path)
  const result = new Array<string>()
  // 去重
  for (const currentPath of selectPaths) {
    // 检查当前路径是否被 result 中已有的路径包含
    if (!result.some((parent) => isSubPath(parent, currentPath))) {
      result.push(currentPath)
    }
  }
  return result
}

function isSubPath(parent: string, child: string): boolean {
  if (child.startsWith(parent)) {
    return true
  } else {
    return false
  }
}

function getAllNodeList(node: Node): Array<Node> {
  let result = new Array<Node>()
  result.push(node)
  for (let child of node.childNodes) {
    result.push(...getAllNodeList(child))
  }
  return result
}

function getNoSelectPathList(): Array<string> {
  let node = treeRef.value?.getNode(rootPath.value)
  if (!node) {
    return []
  }
  let allNodeList = getAllNodeList(node)

  return allNodeList
    .filter((each) => !each.indeterminate && !each.checked)
    .map((each) => each.data.path)
  // console.log(allNodeList)
  // return []
}

defineExpose({
  getSelectPathList,
  getNoSelectPathList
})
</script>
