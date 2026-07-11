---
title: "Architecture Design"
category: "Getting Started"
tags:
  - "getting-started"
date: "2026-07-11"
description: "Architecture Design of loki-auto, including 3-layer architecture, stdio MCP host, background script routing, and WASM sandbox."
order: 11
---

`loki-auto` uses a stateless, event-driven Oneshot CGI paradigm:

![Architecture Diagram](/architecture.jpg)

1. **Layer 1: Host Environment** – Rust Axum MCP server handling Model Context Protocol (MCP) tool bindings and bridging commands over local WebSockets.
2. **Layer 2: Browser Background** – Extension background script managing tab states and routing target instructions.
3. **Layer 3: Browser Sandbox** – The content script instantiates a lightweight WASM-compiled Rhai VM per prompt invocation, executes DOM operations, captures results/logs, and purges the VM from memory instantly.
