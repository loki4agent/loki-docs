---
title: "安装与编译"
category: "快速入门"
tags:
  - "getting-started"
date: "2026-07-11"
description: "loki-auto 浏览器扩展与本地 MCP 服务端的环境准备、编译与构建指南。"
order: 12
---

### 1. 安装浏览器扩展 (Install the Browser Extension)

* **Chrome**: 直接从 [Chrome Web Store](https://chromewebstore.google.com/detail/loki-auto/lhplbecpbfajamlefhaiclmbgjbheclm) 安装。
* **Firefox**: 需要通过源码编译并手动加载（详见下方的 [源码编译与构建](#源码编译与构建-build-from-source) 部分）。

### 2. 启动本地 MCP 服务端 (Launch the Local MCP Server)

启动 Axum 宿主以开始监听浏览器连接和 LLM 工具调用。

您可以直接从 [GitHub Releases](https://github.com/loki4agent/loki-auto/releases) 页面下载适用于您平台的已编译好的 `loki-mcp-server` 二进制文件直接运行，或者通过源码运行：

```bash
# 通过源码启动 MCP 宿主 (默认端口: 10402)
cargo run --bin loki-mcp-server
```

---

### 源码编译与构建 (Build from Source)

如果您想从源码进行编译或针对 Firefox 进行部署，请按照以下步骤操作：

#### 1. 环境准备 (Prerequisites)
请确保您的系统中已安装以下工具：
* [Bun](https://bun.sh/) (工作区包管理器)
* [Rust & Cargo](https://rustup.rs/) (并已安装 `wasm32-unknown-unknown` 目标：`rustup target add wasm32-unknown-unknown`)
* [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) (用于将 Rust 编译为 WASM)

#### 2. 编译与构建 (Compilation and Build)

首先构建 WebAssembly 沙箱，然后编译浏览器扩展资源：

```bash
# 1. 编译 Rust 沙箱为 WebAssembly
cd packages/sandbox
wasm-pack build --target web

# 2. 返回根目录，安装依赖并编译 Chrome/Firefox 扩展
cd ../..
bun install
bun run build
```

这将在项目根目录下生成两个完整编译且自包含的扩展程序目录：
* **Chrome (Manifest V3)**: `./dist/chrome`
* **Firefox (Manifest V2)**: `./dist/firefox`

#### 3. 手动加载浏览器扩展 (Load the Browser Extension Manually)

* **Firefox**: 打开 `about:debugging#/runtime/this-firefox`，点击 **"Load Temporary Add-on..."**，并选择项目根目录下的已编译包 `loki-auto.xpi`（或者选择 `./dist/firefox/manifest.json`）。
* **Chrome**: 打开 `chrome://extensions/`，启用 **开发者模式 (Developer mode)**，点击 **"加载已解压的扩展程序" (Load unpacked)**，然后选择 `./dist/chrome` 文件夹。

---

### 📱 移动端 (Android) 支持与连接

在 Android 手机上，由于无法方便地运行本地守护进程，我们推荐让手机端的浏览器扩展连接至电脑桌面的 MCP 服务端。

您可以使用支持 Chrome MV3 扩展程序的浏览器（如 Kiwi Browser 或 Firefox Beta for Android），并安装 `loki-auto` 扩展。随后选择以下方案之一进行连接通信：

#### 方案 A：使用 ADB 端口反向代理（推荐）
如果您通过 USB 数据线将手机连接到电脑，可以通过 ADB 将手机的 localhost 端口反向映射至电脑：
```bash
adb reverse tcp:10402 tcp:10402
```
* **原理解析**：此命令将手机本地的 `127.0.0.1:10402` 端口流量全部转发到您电脑（宿主机）的 `10402` 端口。
* **优势**：手机端扩展配置无需更改，可直接使用默认的 `ws://127.0.0.1:10402`。

#### 方案 B：局域网连接
如果手机和电脑处于相同的 Wi-Fi 网络下：
1. 修改电脑端 `mcp-server` 配置文件，使 host 监听 `0.0.0.0` 允许外部局域网访问。
2. 在手机浏览器的 `loki-auto` 扩展设置中，将 WebSocket 地址从 `ws://127.0.0.1:10402` 修改为电脑的局域网 IP（例如 `ws://192.168.1.100:10402`）。

