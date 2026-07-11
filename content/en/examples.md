---
title: "Use Case Examples"
category: "Developer Guide"
tags:
  - "developer"
date: "2026-07-11"
description: "Practical automation examples for loki-auto: Reddit posting/replying, Google long-tail search, and Investing.com financial scraping."
order: 23
---

This section provides four common web automation scripts running inside the `loki-auto` sandbox. It demonstrates how to simulate human interactions using synchronous DOM APIs and return structured final results back to the calling LLM.

> [!NOTE]
> Rhai scripts follow the rule where **"the last evaluated expression acts as the return value"**, or you can use an explicit `return` statement. By returning structured objects (such as Maps `#{ ... }` or Booleans), the calling LLM can directly consume the returned payload from the MCP tool response.

### 1. Reddit Posting a New Thread

This script simulates posting a new text thread in a specific subreddit and returns a success/failure status to the LLM:

```rust
log("Starting Reddit posting process...");

let title_selector = "textarea[placeholder='Title']";
let body_selector = "div[role='textbox']";

// 1. Wait for the Title input field to load
if wait_element(title_selector, 8000) {
  // Simulate human typing for the title
  type_as_human(title_selector, "Introduction to loki-auto: The WebAssembly Browser Automation Runtime");
  sleep(1000);
  
  // 2. Locate the rich text body editor and insert content
  if wait_element(body_selector, 5000) {
    type_text(body_selector, "loki-auto is an ultra-lightweight, zero-dependency browser automation tool designed for AI Agents. It runs Rhai scripting language inside a WebAssembly sandbox.");
    sleep(1500);
    
    // 3. Find and click the Submit/Post button
    let submit_btn = "button[type='submit']";
    if wait_element(submit_btn, 5000) {
      click(submit_btn);
      log("Success: Post submit button clicked.");
      return true; // Return success flag to the LLM
    } else {
      log("Error: Could not locate the submit/post button.");
      return false;
    }
  } else {
    log("Error: Could not locate the content body editor.");
    return false;
  }
} else {
  log("Error: Could not locate the Title field. Verify if page has redirected to submit view.");
  return false;
}
```

---

### 2. Reddit Replying to a Question

This script locates the reply input editor on a Reddit post detail page, submits a comment, and reports status to the LLM:

```rust
log("Starting Reddit reply process...");

let reply_editor = "div[role='textbox']";

// 1. Wait for comment textbox to be focusable
if wait_element(reply_editor, 8000) {
  click(reply_editor);
  sleep(500);
  
  // 2. Fill comment reply text
  type_text(reply_editor, "This is an automated reply powered by loki-auto browser runtime. Great discussion here!");
  sleep(1000);
  
  // 3. Find and click submit/comment button
  let submit_btn = "button[type='submit']";
  if wait_element(submit_btn, 5000) {
    click(submit_btn);
    log("Comment posted successfully.");
    return true; // Explicitly return success status
  } else {
    log("Error: Failed to find submit reply button.");
    return false;
  }
} else {
  log("Error: Could not locate reply comment input area.");
  return false;
}
```

---

### 3. Google Searching Long-tail Keywords

This script submits a Google Search query using simulated human keystroke delays to bypass bot detection, returning the operational status:

```rust
log("Starting Google Search with long-tail keywords...");

// Google Search input selector (can be textarea or input depending on page variant)
let search_input = "textarea[name='q']";
if !element_exists(search_input) {
  search_input = "input[name='q']";
}

// 1. Wait for search box to be ready
if wait_element(search_input, 5000) {
  // 2. Type with human-like delays to avoid detection
  type_as_human(search_input, "loki-auto webassembly browser automation github");
  sleep(1200);
  
  // 3. Submit search form or click search button
  let search_btn = "input[name='btnK']";
  if element_exists(search_btn) {
    click(search_btn);
  } else {
    // Fallback standard submit button
    click("button[type='submit']");
  }
  log("Search query submitted successfully.");
  return true; // Return true to indicate search has been initiated
} else {
  log("Error: Search input not found.");
  return false;
}
```

---

### 4. Scraping Financial Data from Investing.com

This script fetches tickers or commodity data (e.g. Apple stock AAPL or Gold price page), extracts market metrics, and **returns the structured financial data (Map object) directly to the calling LLM**:

```rust
log("Scraping real-time market data from Investing.com...");

// Last price selector on Investing.com instrument pages
let last_price_sel = "span[data-test='instrument-price-last']";

// 1. Wait for quotation box to load
if wait_element(last_price_sel, 10000) {
  // 2. Extract price text
  let price = get_text(last_price_sel);
  log("Latest Price: " + price);
  
  // 3. Extract price change percent
  let change_sel = "span[data-test='instrument-price-change-percent']";
  let change = "";
  if element_exists(change_sel) {
    change = get_text(change_sel);
    log("Price Change %: " + change);
  }
  
  // 4. Fetch the metrics grid text
  let metrics_sel = "div[data-test='key-metrics']";
  let metrics_text = "";
  if element_exists(metrics_sel) {
    metrics_text = get_text(metrics_sel);
    log("Key Metrics Details:\n" + metrics_text);
  }
  
  // 5. Structure the dynamic data in a Rhai Map and return it to the LLM
  return #{
    "price": price,
    "change_percent": change,
    "metrics": metrics_text
  };
} else {
  log("Error: Quotation element loading timed out. Verify URL.");
  return #{
    "error": "TimeoutError"
  };
}
```
