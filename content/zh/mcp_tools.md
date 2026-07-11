---
title: "MCP 工具规范"
category: "开发指南"
tags:
  - "developer"
date: "2026-07-11"
description: "loki-auto 暴露的标准模型上下文协议 (MCP) 工具。"
order: 21
---

loki-auto 暴露了标准的模型上下文协议 (Model Context Protocol, MCP) 工具：

* `list_tabs` - 列出所有打开的浏览器标签页及其焦点状态。
* `open_tab(url, active)` - 使用指定的 URL 打开一个新标签页。
* `close_tab(tab_id)` - 通过 ID 关闭目标标签页。
* `activate_tab(tab_id)` - 将浏览器标签页激活并切换到前台。
* `execute_loki_oneshot(target_tab_id, target_url_pattern, rhai_script, payload)` - 在目标标签页的沙箱虚拟机内同步运行 Rhai 自动化脚本。
