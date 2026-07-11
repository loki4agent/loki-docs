---
title: "Security & Isolation"
category: "Security & Isolation"
tags:
  - "security"
date: "2026-07-11"
description: "Security model and sandboxing design of loki-auto."
order: 31
---

`loki-auto` prioritizes user data privacy and session security. In addition to compiler-level WASM sandboxing, the platform implements an end-to-end **Zero-Trust Authorization System**.

### Core Security Lines

* **Strict Sandbox**: The Rhai runtime inside WebAssembly has zero access to standard browser JS APIs, cookies, localStorage, or system files unless explicitly exposed through bridge bindings.
* **No Network Access**: The sandbox cannot initiate `fetch` or AJAX requests. All operations occur locally within the page context, returning results through secure browser extension communication channels.

### Tab-Level Authorization Control

To prevent LLMs from autonomously accessing or interacting with sensitive web pages (such as online banking, personal emails, or social media consoles), `loki-auto` introduces a strict **Tab-level authorization gating mechanism**:

#### 1. How It Works
When an external LLM triggers an `execute_loki_oneshot` command over the MCP protocol, the privileged background script of the browser extension intercepts the request:
* It checks if the target `tabId` is present in the `authorizedTabIds` whitelist array stored in the extension's secure local storage (`chrome.storage.local`).
* If the Tab ID is not whitelisted, the background script blocks the instruction and returns a security error:
  `"Security Blocked: Script execution is locked for this tab..."`
* Under unauthorized status, the Rhai VM is never instantiated in the target tab, ensuring a hard physical lock.

#### 2. User-in-the-Loop Consent
The user retains absolute control. To run an automation script on any tab, the user must explicitly grant permission:
1. Open the extension popup/options and go to **"Loki Playground"**.
2. A tab management panel lists all active browser tabs with an **Authorization Switch**.
3. The target Tab ID is whitelisted *only* when the user toggles the switch on.
4. When a whitelisted tab is closed, its permission is immediately revoked from storage (`chrome.tabs.onRemoved`), ensuring the lease of authority is transient and follows the principle of least privilege.

#### 3. Protection Against Attacks
Even if the WebSocket port or the local MCP server is compromised by malicious entities, attackers cannot hijack arbitrary browser sessions or sensitive sites. This Tab-level isolation boundary establishes a robust defense for secure browser automation.
