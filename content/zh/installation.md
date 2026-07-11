---
title: "安装与编译"
category: "快速入门"
tags:
  - "getting-started"
date: "2026-07-11"
description: "loki-auto 浏览器扩展与本地 MCP 服务端的环境准备、编译与构建指南。"
order: 12
---

### 1. 环境准备 (Prerequisites)
请确保您的系统中已安装以下工具：
* [Bun](https://bun.sh/) (工作区包管理器)
* [Rust & Cargo](https://rustup.rs/) (并已安装 `wasm32-unknown-unknown` 目标：`rustup target add wasm32-unknown-unknown`)
* [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) (用于将 Rust 编译为 WASM)

### 2. 编译与构建 (Compilation and Build)

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

### 3. 加载浏览器扩展 (Load the Browser Extension)

* **Firefox**: 打开 `about:debugging#/runtime/this-firefox`，点击 **"Load Temporary Add-on..."**，并选择项目根目录下的已编译包 `loki-auto.xpi`（或者选择 `./dist/firefox/manifest.json`）。
* **Chrome**: 打开 `chrome://extensions/`，启用 **开发者模式 (Developer mode)**，点击 **"加载已解压的扩展程序" (Load unpacked)**，然后选择 `./dist/chrome` 文件夹。

### 4. 启动本地 MCP 服务端 (Launch the Local MCP Server)

启动 Axum 宿主以开始监听浏览器连接和 LLM 工具调用：

```bash
# 启动 MCP 宿主 (默认端口: 10402)
cargo run --bin loki-mcp-server
```
