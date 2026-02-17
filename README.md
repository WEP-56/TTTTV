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
- **后端** - Rust + Axum
- **桌面框架** - Tauri 2.0
- **播放器** - Hls.js

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

```bash
# 在项目根目录
npm run tauri dev
```

### 构建

```bash
npm run tauri build
```

### 打包发布

构建完成后，会在 `moovie-front/src-tauri/target/release/bundle/` 目录下生成：
- **MSI 安装包** - 带有安装向导的 Windows 安装程序
- **可执行文件** - 直接运行的 .exe 文件

可以直接运行这些文件进行测试或发布。

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