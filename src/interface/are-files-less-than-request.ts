export default class AreFilesLessThanRequest {
  rootPath: string
  num: number
  excludeExts: string[]
  excludePaths: string[]
  enableGitignore: boolean

  constructor(
    rootPath: string,
    num: number,
    excludeExts: string[],
    excludePaths: string[],
    enableGitignore: boolean
  ) {
    this.rootPath = rootPath
    this.num = num
    this.excludeExts = excludeExts
    this.excludePaths = excludePaths
    this.enableGitignore = enableGitignore
  }
}
