# ⚡ 刹那 Chana

> 跨平台截图 & 录屏一体化工具 — 点击、捕捉、分享，一气呵成。

---

## 功能

### 截图
- **多种截图模式**：全屏、区域、窗口、延时、滚动长截图
- **即时标注**：矩形、箭头、文字、马赛克、序号标注
- **贴图 (Pin)**：截图悬浮在屏幕上，支持缩放/旋转/透明度调节
- **剪贴板/本地保存**：截图自动复制到剪贴板，一键保存为 PNG/JPEG/WebP

### 录屏
- **录制模式**：全屏、区域、窗口，最高支持 4K
- **音频采集**：系统音频 + 麦克风，可独立开关
- **硬件加速编码**：NVENC / AMF / VideoToolbox，低 CPU 占用
- **输出格式**：MP4 (H.264/H.265)、GIF、WebM

### 智能增强
- **本地 OCR**：PaddleOCR ONNX 引擎，离线识别中英文文字，截图即可复制
- **二维码识别**：识别截图中二维码/条形码并跳转
- **图像美化**：一键添加圆角、阴影、背景色

### 效率
- **全局快捷键**：一键触发截图/录屏，无需离开当前工作
- **系统托盘**：常驻后台，随时待命
- **自定义工作流**：截图后自动执行标注→保存→复制→通知

---

## 使用场景

| 场景 | 典型操作 |
|------|---------|
| 开发者写文档 | 截图 → 标注箭头/序号 → 贴图到屏幕 → 对照编写 |
| 设计师审稿 | 区域截图 → 矩形标注问题 → 保存反馈 |
| 远程沟通 | 录屏 30 秒 → 生成 GIF → 发送 Slack/飞书 |
| 教程制作 | 区域录屏 + 麦克风 → MP4 输出 → 直接上传 |
| 资料整理 | 滚动长截图网页 → OCR 提取文字 → 粘贴到笔记 |
| 日常记录 | 快捷键截图 → 自动保存 → 无需任何额外操作 |

---

## 跨平台

| 平台 | 屏幕捕获 | 录屏加速 | 安装方式 |
|------|---------|---------|---------|
| **macOS** (13+) | ScreenCaptureKit | VideoToolbox 硬件编码 | DMG / Homebrew |
| **Windows** (10+) | DXGI Desktop Duplication + WGC | NVENC / AMF | MSI / Winget |
| **Linux** (Wayland/X11) | PipeWire + XDG Portal | VAAPI | AppImage / deb / rpm / AUR |

三平台**功能一致、快捷键一致、文件格式一致**。一个肌肉记忆，走遍所有系统。

---

## 独立使用

Chana 是**完全独立的桌面应用**，不依赖任何云服务：

- **OCR** 内嵌 PaddleOCR ONNX 模型（~10MB），安装即用，离线识别
- **录屏** 使用系统原生硬件编码器，不依赖外部 FFmpeg
- **截图** 通过系统 API 直接读取帧缓冲，零网络请求
- **配置** 所有设置保存在本地文件，无遥测、不上传

首次安装后即可离线使用全部功能，无需注册、登录、联网。

---

## 安装

### macOS
```bash
brew install --cask chana
# 或从 GitHub Release 下载 .dmg
```

### Windows
```powershell
winget install Chana
# 或从 GitHub Release 下载 .msi
```

### Linux
```bash
# AppImage（推荐）
wget https://github.com/chana-rs/chana/releases/latest/download/Chana.AppImage
chmod +x Chana.AppImage && ./Chana.AppImage

# 或通过包管理器
sudo apt install chana         # Debian/Ubuntu
sudo dnf install chana         # Fedora
yay -S chana                   # Arch (AUR)
```

---

## 路线图

- [x] **v0.1.0 Demo** — 屏幕捕获 + GUI 骨架
- [ ] **v0.2.0** — 选区覆盖层 + 标注工具栏
- [ ] **v0.3.0** — 全局快捷键 + 剪贴板 + 本地保存
- [ ] **v0.5.0 MVP** — 录屏 + 贴图 + 基础 OCR
- [ ] **v1.0.0** — 三平台正式发布

详见 [产品研发计划](https://github.com/chana-rs/chana/blob/main/docs/ROADMAP.md)

---

## 许可证

Chana 采用 **GNU Affero General Public License v3.0 (AGPL-3.0)**。

- ✅ 个人使用、企业内部使用 — 完全自由
- ✅ AI Agent 脚本调用、自动化集成 — 允许
- ✅ 修改源代码、二次分发 — 允许（需以相同许可开源）
- ❌ 闭源商业化、重新打包售卖 — 禁止
- ❌ 作为 SaaS 服务提供而不开源 — 禁止

完整协议见 [LICENSE](./LICENSE)。
