# 🚀 ChatGPT Linux Desktop
### Blazing-Fast Native ChatGPT Client for Linux

> ⚡ Ultra-lightweight • 🔒 Privacy-focused • 🐧 Built for Linux power users  
> Powered by **Tauri + Rust**

![Linux](https://img.shields.io/badge/Linux-Native-success)
![Tauri](https://img.shields.io/badge/Tauri-Rust-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

---

## 🎯 The Problem

Using ChatGPT inside a browser introduces:

- ❌ High RAM consumption  
- ❌ Background tab overhead  
- ❌ Extension interference  
- ❌ Slower launch time  
- ❌ Distracting UI  

Linux users deserve a clean, native, resource-efficient experience.

---

## 💡 The Solution

A minimal, native Linux desktop client built with **Tauri**.

Instead of bundling Chromium (like Electron apps), this application uses your system’s native WebView.

That means:

- ⚡ Faster startup  
- 🧠 Lower RAM usage  
- 📦 Smaller binary size  
- 🔥 Native performance  

---

## 🧪 Performance Snapshot

| Environment | Idle RAM Usage |
|-------------|----------------|
| Chrome Tab  | ~350–600 MB |
| Electron App | ~250–400 MB |
| **Tauri App** | **~90–150 MB** |

*(Measured on Ubuntu 22.04 – WebKit backend)*

---

## ✨ Features

- Full ChatGPT functionality  
- Native Linux window (no browser chrome)  
- Ultra-light runtime footprint  
- Fast launch  
- Minimal binary size  
- Clean distraction-free interface  
- No telemetry  
- No embedded Chromium  

> Voice features intentionally excluded to preserve minimal system footprint.

---

## 📦 Installation

### 🐧 Debian-Based (.deb)

```bash
sudo dpkg -i chatgpt-linux_1.0.0_amd64.deb
sudo apt -f install

RPM-Based (.rpm
sudo rpm -i chatgpt-linux-1.0.0-1.x86_64.rpm

📦 AppImage (Portable)
chmod +x ChatGPT-Linux.AppImage
./ChatGPT-Linux.AppImage

No installation required.

🛠 Build From Source
Requirements
Node.js (LTS recommended)
Rust toolchain
Tauri CLI
Clone Repository

git clone https://github.com/emadjawad/chatgpt-linux-desktop.git

cd chatgpt-linux-desktop

Install Dependencies
npm run tauri dev

Development Mode
npm run tauri dev

Production Build
npm run tauri build

Build outputs:
.deb
.rpm
.AppImage

🧠 Architecture
Framework: Tauri
Backend: Rust
Frontend: Vite + TypeScript
Rendering: Native WebView (WebKit-based on Linux)
Zero Chromium bundling.
Maximum efficiency

🔐 Security & Privacy
No credential storage
Authentication handled directly by official ChatGPT interface
No telemetry
No tracking
No API key required
This project is a native desktop wrapper focused purely on performance and efficiency.

🗺 Roadmap
[ ] Auto-update support
[ ] System tray integration
[ ] Custom keyboard shortcuts
[ ] Theming support
[ ] Multi-account profiles
⭐ Support The Project
If you find this useful:
⭐ Star the repository
🐛 Report issues
💡 Suggest features
🔧 Submit pull requests
Growing this project helps improve native tooling for Linux users.

⚠ Disclaimer
This project is independent and not affiliated with or endorsed by OpenAI.
ChatGPT is a trademark of OpenAI.

📜 License
MIT License — Free to use, modify, and distribute.
