# 👻 GhostCursor
> A local-first, AI-augmented terminal wrapper for Kali Linux.

GhostCursor sits silently in your PTY, sniffing for errors. When a command fails or permission is denied, a local **TinyLlama-1.1B** model wakes up, analyzes the context, and provides a color-coded fix—all without your data ever leaving your machine.



## ✨ Features
- **Zero-Latency Sniffing:** Uses `portable-pty` to intercept stderr/stdout in real-time.
- **Smart Colors:** - 🟢 **Green:** Suggestions for typos and "Command Not Found."
  - 🔴 **Red:** Solutions for "Permission Denied" and system errors.
- **Privacy First:** Powered by `llama_cpp`, running a GGUF model locally on your CPU.
- **Hardware Optimized:** Custom build flags for maximum compatibility with Virtual Machines.

## 🚀 Quick Start

### 1. Requirements
- Rust 1.80+
- A GGUF model (e.g., `ghost-brain.gguf`) in the `./models` directory.

### 2. Installation
```bash
git clone [https://github.com/your-username/ghost-cursor](https://github.com/your-username/ghost-cursor)
cd ghost-cursor
cargo build --release


