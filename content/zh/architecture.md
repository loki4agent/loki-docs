---
title: "架构设计"
category: "快速入门"
tags:
  - "getting-started"
date: "2026-07-11"
description: "loki-auto 架构设计：三层架构、stdio MCP 宿主、扩展 background 脚本标签路由与 WASM Rhai VM 沙箱。"
order: 11
---

`loki-auto` 采用无状态、事件驱动的 Oneshot CGI 范式：

![架构拓扑图](/architecture.jpg)

1. **第一层：宿主环境 (Layer 1: Host Environment)** – Rust Axum MCP 服务端，负责处理模型上下文协议 (MCP) 工具绑定，并通过本地 WebSocket 桥接指令。
2. **第二层：浏览器后台 (Layer 2: Browser Background)** – 扩展程序的后台脚本 (Background Script)，负责管理标签页状态并路由目标指令。
3. **第三层：浏览器沙箱 (Layer 3: Browser Sandbox)** – 内容脚本 (Content Script) 在每次 prompt 调用时按需实例化一个轻量级的、经 WASM 编译的 Rhai 虚拟机 (VM)，执行 DOM 操作，捕获执行结果与日志，并在执行完成后立即从内存中卸载清除。
