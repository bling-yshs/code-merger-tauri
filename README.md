<p align="center"><img src="https://cdn.jsdelivr.net/gh/bling-yshs/code-merger-tauri@master/backup-resouces/app-icon.svg" width="300" alt="icon" /></p>
<p align="center"><b>基于 <a href="https://tauri.app/">Tauri</a> 代码合并工具</b></p>
<p align="center">
  <a href="https://www.gnu.org/licenses/gpl-3.0.html"><img src="https://img.shields.io/github/license/bling-yshs/code-merger-tauri" alt="License"></a>
  <a href="https://github.com/bling-yshs/code-merger-tauri"><img src="https://img.shields.io/github/stars/bling-yshs/code-merger-tauri?style=flat" alt="Stars"></a>
  <a href="https://github.com/badges/shields/pulse"><img src="https://img.shields.io/github/commit-activity/m/bling-yshs/code-merger-tauri" alt="Activity"/></a>
  <a href="https://github.com/bling-yshs/code-merger-tauri/releases"><img src="https://img.shields.io/github/v/release/bling-yshs/code-merger-tauri" alt="GitHub release"></a>
</p>


## 基本信息

### 功能介绍

您可以选择一个文件夹，点击“开始合并”按钮以后，可以将其内部的所有文本文件合并为一条字符串

![features](https://cdn.jsdelivr.net/gh/bling-yshs/code-merger-tauri@master/docs/img/features.png)

![result](https://cdn.jsdelivr.net/gh/bling-yshs/code-merger-tauri@master/docs/img/result.png)

### 项目特色

- 支持在属性选择器内手动取消选择某些子文件/文件夹
- 树形选择器支持延迟加载，速度飞快，不卡顿
- 支持后缀名过滤
- 子文件数量检测，防止误操作

### 系统支持

Windows 10+、MacOS、Linux

### 架构支持

x86_64、Arm（除Linux，因为交叉编译有问题）

## 快速开始

### 下载

- ✅ [稳定版](https://github.com/bling-yshs/code-merger-tauri/releases/latest)
- 🚀 [测试版](https://github.com/bling-yshs/code-merger-tauri/releases/tag/v9.9.9)（根据最新 Commit 构建）

#### 我应该下载什么版本？

普通 Windows 用户请下载 `x64-setup.exe` 结尾的文件，M系列芯片的苹果用户请下载 `aarch64.dmg` 结尾的文件

### 使用

[视频教程]()

## 项目开发

### 语言和框架

Tauri + TypeScript + Vue 3 + Rust

### IDE

[RustRover](https://www.jetbrains.com/rust/) 或 [VS Code](https://code.visualstudio.com/)

### 格式化

在 Pull Request 之前，请务必使用 rustfmt 进行代码格式化，命令：`cargo fmt`
