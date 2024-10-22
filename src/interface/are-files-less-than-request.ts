export default class AreFilesLessThanRequest {
  rootPath: string
  num: number
  noSelectedPaths: string[]
  excludeDirs: string[]
  excludeExts: string[]
  enableGitignore: boolean
  
  constructor(rootPath: string, num: number, noSelectedPaths: string[], excludeDirs: string[], excludeExts: string[], enableGitignore: boolean) {
    this.rootPath = rootPath
    this.num = num
    this.noSelectedPaths = noSelectedPaths
    this.excludeDirs = excludeDirs
    this.excludeExts = excludeExts
    this.enableGitignore = enableGitignore
  }
}
