# TTTTV

Windows 端，Win11 原生软件风格的聚合全网资源的影视搜索、观看器。

![TTTTV Logo](./logo.png)

界面演示：[BiliBili专栏](https://www.bilibili.com/opus/1170268044355698696)

## 特性

- 🎬 **聚合搜索** - 支持多个资源站的搜索和观看
- 🎨 **Win11 风格** - 原生 UI 设计，美观简洁
- 📺 **HLS 播放** - 流畅的 HLS 视频播放
- ⏱️ **播放历史** - 自动保存播放进度，支持续播
- ⭐ **收藏功能** - 收藏喜欢的视频
- 🔍 **搜索历史** - 保存搜索记录，方便回顾
- 🎞️ **M3U8 直连** - 支持输入 m3u8 链接直接播放
- 🌙 **主题切换** - 支持浅色/深色/跟随系统主题
- ⚙️ **资源管理** - 可配置启用/禁用资源站，添加自定义源

## 技术栈

- **前端** - Vue 3 + TypeScript + Element Plus
- **后端** - Rust + Axum (独立服务器)
- **桌面框架** - Tauri 2.0 (前端容器)
- **播放器** - Hls.js

## 项目结构

```
TTTTV/
├── Moovie/              # 后端服务器 (完整功能)
│   ├── config/          # 资源站配置
│   ├── src/             # 后端源码
│   └── Cargo.toml
└── moovie-front/        # 前端 (Tauri)
    ├── src/             # 前端源码
    ├── src-tauri/       # Tauri 容器
    └── package.json
```

## 快速开始

### 前置要求

- Node.js 18+
- Rust 1.75+
- Tauri 2.0 CLI

### 安装依赖

```bash
# 前端
cd moovie-front
npm install

# 后端 (Rust 会自动处理)
```

### 开发模式

**重要：需要同时运行两个程序！**

1. **启动后端服务器** (新终端)
```bash
cd Moovie
cargo run
```
后端将运行在 http://127.0.0.1:5007

2. **启动前端** (另一个新终端)
```bash
cd moovie-front
npm run tauri dev
```

### 构建

#### 1. 构建后端
```bash
cd Moovie
cargo build --release
```
后端可执行文件位置：`Moovie/target/release/moovie.exe`

#### 2. 构建前端 (Tauri)
```bash
cd moovie-front
npm run tauri build
```
前端安装包位置：`moovie-front/src-tauri/target/release/bundle/msi/`

### 打包发布

#### 方式一：同时分发两个文件 (推荐)
构建完成后，需要同时提供以下两个文件：
1. **后端服务器**：`Moovie/target/release/moovie.exe`
2. **前端安装包**：`moovie-front/src-tauri/target/release/bundle/msi/*.msi`

**使用说明**：
1. 用户需要先运行 `moovie.exe` (保持后台运行)
2. 然后安装并运行 TTTTV 前端

#### 方式二：手动打包在一起
1. 将 `Moovie/target/release/moovie.exe` 和 `config/` 文件夹复制到一个目录
2. 可以创建一个启动脚本同时启动后端和前端

**重要**：`config/sources.json` 必须和后端可执行文件在同一目录下！

---

## 版权声明

TTTTV 尊重知识产权，遵守《数字千年版权法案》(DMCA) 及相关法律法规。

### 免责声明

本软件不存储任何视频文件，所有内容均来自互联网公开资源。本软件仅提供搜索和索引服务，不对任何第三方提供的内容负责。

### 版权投诉

如果您是版权所有者，认为本软件索引的内容侵犯了您的权益，请通过 issues 页面提交投诉，制作者将在 24-48 小时内处理。

投诉时请提供：
- 您的联系方式
- 侵权内容的具体链接
- 版权所有权证明

## GitHub

[https://github.com/WEP-56/TTTTV](https://github.com/WEP-56/TTTTV)

## 许可证

MIT License

---

**本项目仅供学习和参考，请勿分发、商用。**