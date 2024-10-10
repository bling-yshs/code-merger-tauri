export default class MergeFilesRequest {
  constructor(rootPath: string, excludeExts: Array<string>, excludePaths: Array<string>) {
    this.rootPath = rootPath
    this.excludeExts = excludeExts
    this.excludePath = excludePaths
  }

  rootPath: string
  excludeExts: Array<string>
  excludePath: Array<string>
}
