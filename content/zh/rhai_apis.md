---
title: "Rhai 沙箱 DOM APIs"
category: "开发指南"
tags:
  - "developer"
date: "2026-07-11"
description: "Rhai 自动化脚本在 WASM 沙箱中可访问的同步 DOM 钩子及 API 规范。"
order: 22
---

在 WASM 沙箱内运行的脚本可以访问以下同步 DOM 钩子：

* `sleep(ms)` - 挂起执行指定的毫秒数。
* `log(message)` - 向执行控制台输出消息/对象。
* `dom_to_string() -> String` - 返回当前页面经过清理与 Token 优化（LLM 友好型）的 HTML 树。
* `element_exists(selector) -> bool` - 立即检查是否存在匹配 CSS 选择器的元素。
* `wait_element(selector, [timeout_ms]) -> bool` - 阻塞执行直至元素出现，或在超时后抛出异常。
* `click(selector, [timeout_ms]) -> bool` - 等待并点击选中的元素。
* `type_text(selector, text, [timeout_ms]) -> bool` - 等待并立即向 input 或 textarea 设置文本，触发输入事件（推荐用于长文本/文章录入）。
* `type_as_human(selector, text, [timeout_ms]) -> bool` - 等待并逐字模拟人工输入文本，带有随机化的人类输入延迟以及标准的 keydown/keyup 事件（推荐用于搜索框和登录输入）。
* `get_text(selector, [timeout_ms]) -> String` - 提取元素的内部文本内容。
* `get_value(selector, [timeout_ms]) -> String` - 提取表单输入框的值。
* `get_attribute(selector, attr, [timeout_ms]) -> String` - 提取特定属性的值。
* `scroll_to(selector, [timeout_ms]) -> bool` - 平滑滚动视口到目标元素位置。
* `get_loki_data(dom_selector, [timeout_ms]) -> String` - 限定目标 DOM 范围并提取带有 `data-loki` 属性的子节点，返回干净且易于 Markdown 解析的格式。

> [!NOTE]
> DOM API 中的 `[timeout_ms]` 参数（例如 `click`、`type_text`、`type_as_human`）指定了**在抛出 `TimeoutError` 之前等待目标元素在 DOM 中出现的最大时间**（默认值为 `5000` 毫秒）。对于 `type_as_human`，该超时仅指元素检测的上限，并不限制打字本身执行的时间（打字是异步执行的，仅受 45 秒全局 MCP 单次任务超时的限制）。

### Rhai 脚本示例 (Example Rhai Script)
```rust
print("Interacting with search input...");
if wait_element("textarea[name='q']", 2000) {
  type_text("textarea[name='q']", "weather in Boston today");
  sleep(500);
  click("input[name='btnK']");
}
```
