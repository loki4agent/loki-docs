---
title: "Privacy Policy"
category: "Getting Started"
tags:
  - "privacy"
date: "2026-07-11"
description: "Official Privacy Policy for loki-auto browser extension."
order: 99
---

This Privacy Policy explains how the `loki-auto` browser extension handles, processes, and protects your data.

Our core commitment: **Privacy is our fundamental design principle. Loki-auto does NOT collect, upload, or share any of your personal data.**
---
### 1. Data Collection Policy (Zero Data Collection)

`loki-auto` is a **local-first** browser automation runtime.
* **No Personal Data Collection**: We do not collect your name, email address, IP address, credentials, or any other personally identifiable information (PII).
* **No Browsing History Collection**: We do not record, monitor, or track your browsing history or activity across the web.
* **No Keystroke/Scraped Data Logging**: Any input simulated during automation or text extracted from web pages is processed transiently in local memory and is never intercepted by us or any third parties.
---
### 2. Data Processing & Storage (Local-Only Processing)

All script executions and data manipulation happen strictly on your local machine:
* **Local WASM Sandbox**: Rhai automation scripts compile and execute inside a secure WebAssembly (WASM) virtual machine sandbox directly within your browser's page context.
* **Local Whitelist Storage**: The Tab IDs you explicitly authorize (`authorizedTabIds`) are saved exclusively in your browser's local secure storage (`chrome.storage.local`) and are never sent to external servers.
* **No Outbound Sandbox Traffic**: The WebAssembly execution sandbox operates under a zero network policy and cannot make outbound HTTP, AJAX, or WebSocket requests.
---
### 3. Third-Party Sharing & Tracking (No Third-Party Sharing)

* **No Analytics or Trackers**: This extension contains no advertisements, third-party trackers (e.g., Google Analytics), or analytics SDKs.
* **No Data Monetization**: Because we do not collect any user data, we do not share, sell, or trade your information with any third-party companies, advertisers, or organizations.

---
### 4. Policy Updates

We may update this Privacy Policy from time to time. Any changes will be posted directly on this page. We encourage you to review this policy periodically for updates.

---
### 5. Contact Us

If you have any questions regarding this Privacy Policy or our security model, please contact the maintainers via our open-source page:
* Project Homepage: [GitHub Repository](https://github.com/loki4agent/loki-auto)
* Official Website: [loki4agent.com](https://loki4agent.com)
