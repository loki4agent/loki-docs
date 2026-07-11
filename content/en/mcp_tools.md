---
title: "MCP Tool Specifications"
category: "Developer Guide"
tags:
  - "developer"
date: "2026-07-11"
description: "Model Context Protocol (MCP) tools exposed by loki-auto."
order: 21
---

loki-auto exposes standard Model Context Protocol (MCP) tools:

* `list_tabs` - Lists all open browser tabs and their focused states.
* `open_tab(url, active)` - Opens a new tab with the specified URL.
* `close_tab(tab_id)` - Closes a target tab by ID.
* `activate_tab(tab_id)` - Focuses a browser tab to the foreground.
* `execute_loki_oneshot(target_tab_id, target_url_pattern, rhai_script, payload)` - Runs a Rhai automation script synchronously inside the sandboxed VM of the target tab.
