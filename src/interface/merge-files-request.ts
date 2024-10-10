export default class MergeFilesRequest {
  constructor(rootPath: string, excludeExts: Array<string>, excludePaths: Array<string>) {
    this.rootPath = rootPath
    this.excludeExts = excludeExts
    this.excludePaths = excludePaths
  }

  rootPath: string
  excludeExts: Array<string>
  excludePaths: Array<string>
}
