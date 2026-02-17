# 打包常见问题解决

## 问题 1：Tauri 下载 WiX Toolset 很慢或卡住

### 解决方案：手动下载并安装 WiX Toolset

1. **使用国内镜像下载**
   - 清华镜像：https://mirrors.tuna.tsinghua.edu.cn/github-release/wixtoolset/wix3/
   - 或者直接下载：https://github.com/wixtoolset/wix3/releases/download/wix3141rtm/wix314-binaries.zip

2. **解压并配置环境变量**
   - 解压到某个目录，比如 `C:\wix314`
   - 添加环境变量：`WIX` = `C:\wix314`
   - 把 `C:\wix314` 添加到 PATH

3. **或者，暂时跳过 MSI 打包**
   只构建 exe 不构建 MSI 安装包，修改 `tauri.conf.json`：
   ```json
   "bundle": {
     "active": true,
     "targets": ["nsis", "msi"]  // 或者只保留 "nsis"
   }
   ```

---

## 问题 2：Rust 后端有很多依赖文件

### 解决方案：使用单文件打包（已配置）

项目已配置单文件打包！在 `Moovie/Cargo.toml` 中：
- `lto = true` - 启用链接时优化
- `codegen-units = 1` - 单代码单元
- `strip = true` - 去除调试符号
- Windows 配置：静态链接 CRT

构建命令：
```bash
cd Moovie
cargo build --release
```

最终只有一个 `ttttv.exe` 文件，没有任何依赖！

---

## 问题 3：Cargo 下载依赖慢

### 解决方案：使用国内镜像（已配置）

已在两个地方配置了 rsproxy 镜像：
- `Moovie/.cargo/config.toml`
- `moovie-front/src-tauri/.cargo/config.toml`

使用的是 https://rsproxy.cn 镜像，速度很快！
