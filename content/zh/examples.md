---
title: "实战应用案例"
category: "开发指南"
tags:
  - "developer"
date: "2026-07-11"
description: "loki-auto 实战案例脚本：Reddit 发帖/回复、Google 搜索长尾词与 Investing.com 财经数据提取。"
order: 23
---

本章节提供了四个常见的浏览器自动化任务在 `loki-auto` 沙箱下运行的 Rhai 脚本实现，展示了如何通过同步 DOM 接口模拟人类行为并将最终结果结构化返回给 LLM。

> [!NOTE]
> Rhai 脚本遵循**“最后一条评估表达式即为返回值”**或使用显式 `return` 语句的规则。通过显式 `return` 返回结构化对象（例如 Maps `#{ ... }` 或 Booleans），调用的 LLM 即可在 MCP 工具响应中直接消费返回的数据。

### 1. Reddit 发布新帖子

此脚本模拟自动登录后的用户在 Reddit 特定 Subreddit 提交页发布新文本帖子，并在发布完毕后向 LLM 返回成功/失败状态：

```rust
log("开始执行 Reddit 发帖流程...");

let title_selector = "textarea[placeholder='Title']";
let body_selector = "div[role='textbox']";

// 1. 等待并定位标题输入框
if wait_element(title_selector, 8000) {
  // 模拟人类打字录入标题
  type_as_human(title_selector, "Introduction to loki-auto: The WebAssembly Browser Automation Runtime");
  sleep(1000);
  
  // 2. 定位富文本编辑器区域并填入正文内容
  if wait_element(body_selector, 5000) {
    type_text(body_selector, "loki-auto is an ultra-lightweight, zero-dependency browser automation tool designed for AI Agents. It runs Rhai scripting language inside a WebAssembly sandbox.");
    sleep(1500);
    
    // 3. 寻找并点击发布 submit 按钮
    let submit_btn = "button[type='submit']";
    if wait_element(submit_btn, 5000) {
      click(submit_btn);
      log("发帖成功：提交按钮已点击。");
      return true; // 返回成功标志给 LLM
    } else {
      log("错误：未能定位到发布提交按钮。");
      return false;
    }
  } else {
    log("错误：未能定位到内容编辑框。");
    return false;
  }
} else {
  log("错误：未能定位到标题输入框，请确认页面是否已跳转至 submit 页面。");
  return false;
}
```

---

### 2. Reddit 回复评论/问题

在 Reddit 帖子详情页面中，此脚本定位评论编辑框，模拟自动回复，并向 LLM 反馈提交状态：

```rust
log("开始执行 Reddit 回复评论流程...");

let reply_editor = "div[role='textbox']";

// 1. 等待评论输入框出现并获取焦点
if wait_element(reply_editor, 8000) {
  click(reply_editor);
  sleep(500);
  
  // 2. 输入评论回复内容
  type_text(reply_editor, "This is an automated reply powered by loki-auto browser runtime. Great discussion here!");
  sleep(1000);
  
  // 3. 定位并点击提交评论按钮
  let submit_btn = "button[type='submit']";
  if wait_element(submit_btn, 5000) {
    click(submit_btn);
    log("评论已成功发送。");
    return true; // 显式返回成功状态
  } else {
    log("错误：未能定位评论发布按钮。");
    return false;
  }
} else {
  log("未能定位到评论输入编辑框，请确认是否处于可评论的主题帖页面。");
  return false;
}
```

---

### 3. Google 搜索长尾关键词

此脚本执行 Google 搜索，并使用人类打字模拟以规避反爬系统的自动检测：

```rust
log("开始执行 Google 长尾关键词搜索...");

// Google 搜索框可能为 textarea 或 input
let search_input = "textarea[name='q']";
if !element_exists(search_input) {
  search_input = "input[name='q']";
}

// 1. 等待搜索输入框就绪
if wait_element(search_input, 5000) {
  // 2. 慢速模拟人类打字以穿透反爬识别
  type_as_human(search_input, "loki-auto webassembly browser automation github");
  sleep(1200);
  
  // 3. 寻找搜索提交按钮或表单进行提交
  let search_btn = "input[name='btnK']";
  if element_exists(search_btn) {
    click(search_btn);
  } else {
    // 降级使用标准的 submit 按钮
    click("button[type='submit']");
  }
  log("搜索请求提交成功，等待结果呈现。");
  return true; // 返回给 LLM 表明操作成功完成
} else {
  log("无法定位 Google 搜索输入框。");
  return false;
}
```

---

### 4. Investing.com 获取财经行情数据

此脚本用于从 Investing.com（例如 Apple 或黄金实时行情页面）提取资产的最新成交价、涨跌幅及关键指标数据，并**将提取的动态财经数据结构化（Map 键值对）返回给 LLM**：

```rust
log("正在抓取 Investing.com 财经实时行情数据...");

// Investing.com 对应商品/股票最后一笔成交价的选择器
let last_price_sel = "span[data-test='instrument-price-last']";

// 1. 确保商品最新报价渲染出来
if wait_element(last_price_sel, 10000) {
  // 2. 提取最新价格
  let price = get_text(last_price_sel);
  log("最新成交价: " + price);
  
  // 3. 提取今日波动百分比
  let change_sel = "span[data-test='instrument-price-change-percent']";
  let change = "";
  if element_exists(change_sel) {
    change = get_text(change_sel);
    log("今日涨跌幅: " + change);
  }
  
  // 4. 抓取下方详细的核心指标数据区块
  let metrics_sel = "div[data-test='key-metrics']";
  let metrics_text = "";
  if element_exists(metrics_sel) {
    metrics_text = get_text(metrics_sel);
    log("核心财经数据详情:\n" + metrics_text);
  }
  
  // 5. 将提取的动态数据组成 Map (Rhai 键值对哈希表) 返回给 LLM
  return #{
    "price": price,
    "change_percent": change,
    "metrics": metrics_text
  };
} else {
  log("财经行情数据加载超时，请确认 URL 是否正确。");
  return #{
    "error": "TimeoutError"
  };
}
```
