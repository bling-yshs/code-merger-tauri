export default class MergeFilesRequest {
  constructor(
    rootPath: string,
    excludeExts: Array<string>,
    excludePaths: Array<string>,
    enableGitignore: boolean
  ) {
    this.rootPath = rootPath
    this.excludeExts = excludeExts
    this.excludePaths = excludePaths
    this.enableGitignore = enableGitignore
  }

  rootPath: string
  excludeExts: Array<string>
  excludePaths: Array<string>
  enableGitignore: boolean
}
