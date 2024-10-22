export default class MergeFilesRequest {
  rootPath: string
  noSelectedPaths: Array<string>
  excludeDirs: Array<string>
  excludeExts: Array<string>
  enableGitignore: boolean
  
  constructor(rootPath: string, noSelectedPaths: Array<string>, excludeDirs: Array<string>, excludeExts: Array<string>, enableGitignore: boolean) {
    this.rootPath = rootPath
    this.noSelectedPaths = noSelectedPaths
    this.excludeDirs = excludeDirs
    this.excludeExts = excludeExts
    this.enableGitignore = enableGitignore
  }
}
