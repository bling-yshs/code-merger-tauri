export default class GetSubFilesRequest {
  rootPath: string
  currentPath: string
  
  constructor(root_path: string, current_path: string) {
    this.rootPath = root_path
    this.currentPath = current_path
  }
}
