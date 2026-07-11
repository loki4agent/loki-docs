---
title: "Rhai Sandbox DOM APIs"
category: "Developer Guide"
tags:
  - "developer"
date: "2026-07-11"
description: "Synchronous DOM hooks and API specifications for Rhai automation scripts."
order: 22
---

Scripts running inside the WASM sandbox have access to the following synchronous DOM hooks:

* `sleep(ms)` - Suspends execution for the specified milliseconds.
* `log(message)` - Prints a message/object to the execution console logs.
* `dom_to_string() -> String` - Returns a cleaned, token-optimized (LLM-friendly) HTML tree of the current page.
* `element_exists(selector) -> bool` - Immediate check if a element matches the CSS selector.
* `wait_element(selector, [timeout_ms]) -> bool` - Blocks until the element is present, or throws a timeout exception.
* `click(selector, [timeout_ms]) -> bool` - Wait and click on the selected element.
* `type_text(selector, text, [timeout_ms]) -> bool` - Wait and instantly set text into an input or textarea, triggering input events (recommended for long texts/articles).
* `type_as_human(selector, text, [timeout_ms]) -> bool` - Wait and type text character-by-character with randomized human typing delays and standard keydown/keyup events (recommended for search boxes and login inputs).
* `get_text(selector, [timeout_ms]) -> String` - Extract inner text content.
* `get_value(selector, [timeout_ms]) -> String` - Extract form input value.
* `get_attribute(selector, attr, [timeout_ms]) -> String` - Extract specific attribute value.
* `scroll_to(selector, [timeout_ms]) -> bool` - Scroll the viewport smoothly to the targeted element.
* `get_loki_data(dom_selector, [timeout_ms]) -> String` - Scopes target DOM and extracts child nodes bearing `data-loki` attributes in a clean, Markdown-friendly format.

> [!NOTE]
> The `[timeout_ms]` parameter in DOM APIs (e.g., `click`, `type_text`, `type_as_human`) specifies the **maximum wait time for the target element to appear in the DOM** before throwing a `TimeoutError` (defaults to `5000` ms). For `type_as_human`, this is the element detection limit and does *not* restrict the typing execution time itself (which runs asynchronously and is only limited by the global 45-second MCP oneshot task timeout).

### Example Rhai Script
```rust
print("Interacting with search input...");
if wait_element("textarea[name='q']", 2000) {
  type_text("textarea[name='q']", "weather in Boston today");
  sleep(500);
  click("input[name='btnK']");
}
```
