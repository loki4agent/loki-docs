---
title: "安全与沙箱隔离"
category: "安全与隔离"
tags:
  - "security"
date: "2026-07-11"
description: "loki-auto 的安全模型与沙箱隔离设计。"
order: 31
---

`loki-auto` 在设计之初就将用户的数据隐私和会话安全放在首位。除了代码级别的 WASM 沙箱外，系统还建立了端到端的**零信任授权防御体系**。

### 核心安全防线

* **严格沙箱限制 (Strict Sandbox)**: WebAssembly 内部的 Rhai 运行时无法访问标准的浏览器 JS API、Cookie、localStorage 或系统文件，除非通过桥接绑定显式暴露。
* **零网络访问 (No Network Access)**: 沙箱无法发起 `fetch` 或 AJAX 请求。所有操作均在页面上下文中本地发生，结果通过安全的浏览器扩展通信通道返回。

### Tab 级授权控制 (Tab-Level Authorization)

为了防止 LLM 在不受控制的情况下私自操作用户的敏感网页（如网银、个人邮箱、社交媒体管理后台等），`loki-auto` 引入了严格的 **Tab 级授权控制机制**：

#### 1. 工作原理
当外部 LLM 通过 MCP 协议下发 `execute_loki_oneshot` 自动化指令时，浏览器扩展程序的特权后台脚本（Background Context）会对目标标签页进行拦截审计：
* 检查目标 Tab ID 是否存在于浏览器安全存储（`chrome.storage.local`）的 `authorizedTabIds` 白名单列表中。
* 如果该 Tab ID 未被列入白名单，后台脚本将拒绝转发指令，并返回安全错误：
  `"Security Blocked: Script execution is locked for this tab..."`
* 沙箱环境在未授权状态下，Rhai VM 根本不会被实例化，从而在物理上切断了非授权代码的执行路径。

#### 2. 用户自主控制
用户拥有绝对的控制权。若要对某个特定标签页执行自动化脚本，用户必须在浏览器中：
1. 点击扩展程序图标并进入 **"Loki Playground" (配置选项页)**。
2. 系统会列出当前所有的活动标签页，每一个标签页旁均配有**白名单授权开关**。
3. 只有当用户主动勾选/开启特定标签页的授权时，该 Tab ID 才会加入 `authorizedTabIds` 白名单。
4. 一旦标签页被关闭，该 Tab ID 会立即从白名单中自动抹除（通过 `chrome.tabs.onRemoved` 监听器），确保授权生命周期的瞬时性与最小化。

#### 3. 防范恶意利用
即使底层的 WebSocket 连接或 MCP Server 宿主被恶意代码挟持，攻击者也无法绕过浏览器内核的沙箱保护去操控未授权的页面。这种基于 Tab ID 的物理隔离机制构建了浏览器自动化领域极其强大的安全防线。
