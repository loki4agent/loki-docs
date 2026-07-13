---
title: "Installation & Compilation"
category: "Getting Started"
tags:
  - "getting-started"
date: "2026-07-11"
description: "Prerequisites, compilation and build instructions for loki-auto browser extension and local MCP server."
order: 12
---

### 1. Install the Browser Extension

* **Chrome**: Install directly from the [Chrome Web Store](https://chromewebstore.google.com/detail/loki-auto/lhplbecpbfajamlefhaiclmbgjbheclm).
* **Firefox**: Build from source and load manually (see [Build from Source](#build-from-source) below).

### 2. Launch the Local MCP Server

Start the Axum host to begin listening for browser connections and LLM tool calls.

You can download the pre-compiled `loki-mcp-server` binary for your platform from [GitHub Releases](https://github.com/loki4agent/loki-auto/releases) and run it directly, or run it from source:

```bash
# Start the MCP host from source (default port: 10402)
cargo run --bin loki-mcp-server
```

---

### Build from Source (Development / Firefox)

#### 1. Prerequisites
Ensure you have the following installed on your system:
* [Bun](https://bun.sh/) (Workspace package manager)
* [Rust & Cargo](https://rustup.rs/) (with `wasm32-unknown-unknown` target installed: `rustup target add wasm32-unknown-unknown`)
* [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) (for Rust-to-WASM compilation)

#### 2. Compilation and Build

Build the WebAssembly sandbox first, and then compile the extension assets:

```bash
# 1. Compile Rust Sandbox to WebAssembly
cd packages/sandbox
wasm-pack build --target web

# 2. Return to root, install dependencies and compile the Chrome/Firefox extensions
cd ../..
bun install
bun run build
```

This will output two fully-compiled, self-contained extension directories under the root folder:
* **Chrome (Manifest V3)**: `./dist/chrome`
* **Firefox (Manifest V2)**: `./dist/firefox`

#### 3. Load the Browser Extension Manually

* **Firefox**: Open `about:debugging#/runtime/this-firefox`, click **"Load Temporary Add-on..."**, and select the compiled package `loki-auto.xpi` at the root of the project (or select `./dist/firefox/manifest.json`).
* **Chrome**: Open `chrome://extensions/`, enable **Developer mode**, click **"Load unpacked"**, and select the `./dist/chrome` folder.

---

### 📱 Mobile (Android) Support & Connection

On Android devices, since running local background daemons is restricted by the OS, we recommend connecting the mobile browser extension to the MCP server running on your desktop computer.

Install the `loki-auto` extension in an Android browser that supports Manifest V3 extensions (such as Kiwi Browser or Firefox Beta for Android), and choose one of the following methods to establish a connection:

#### Scheme A: ADB Port Reverse Port Forwarding (Recommended)
If your mobile device is connected to your computer via USB, use ADB to forward localhost connections back to your host machine:
```bash
adb reverse tcp:10402 tcp:10402
```
* **How it works**: This maps `127.0.0.1:10402` on the Android device to port `10402` on your computer.
* **Advantage**: The mobile extension settings do not need to be changed; it can use the default `ws://127.0.0.1:10402`.

#### Scheme B: LAN Connection
If both devices are connected to the same local Wi-Fi network:
1. Configure the `mcp-server` host settings to listen on `0.0.0.0` (to allow external LAN connections).
2. In your mobile browser's `loki-auto` extension options, change the WebSocket URL from `ws://127.0.0.1:10402` to your computer's local IP address (e.g., `ws://192.168.1.100:10402`).

