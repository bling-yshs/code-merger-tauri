export default class GetSubFilesRequest {
  rootPath: string
  currentPath: string
  
  constructor(rootPath: string, currentPath: string) {
    this.rootPath = rootPath
    this.currentPath = currentPath
  }
}
