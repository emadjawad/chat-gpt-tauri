# ChatGPT Desktop (Tauri Edition)

**ChatGPT Desktop** is a lightweight desktop application for ChatGPT built using **Tauri**, **Rust**, and **Vite**.  
This project was created because **OpenAI does not provide an official graphical user interface (GUI) for ChatGPT on Linux distributions**. This desktop application fills that gap by offering a native Linux desktop experience with minimal system resource usage.

---

## Why Was This Project Created?

Currently, OpenAI provides access to ChatGPT through a web-based interface only, requiring a browser to interact with the AI model. While this works well on most platforms, Linux users are left without an official native application.

### The Challenge:

There is no official desktop application for ChatGPT available for Linux-based distributions such as:
- **Ubuntu**
- **Fedora**
- **Arch**
- And other Linux distros.

### The Solution:

This project was built to solve that problem. By using **Tauri** (a lightweight app framework) and **Rust**, we have created a native Linux desktop application for ChatGPT. The app is designed to be as minimal as possible while maintaining all the core features of ChatGPT.

With this app, you get:

- **Seamless ChatGPT experience directly on your desktop**.
- **Lightweight and fast** application that doesn’t require the overhead of a browser.
- **Resource efficiency**, designed to use as little CPU and RAM as possible.

---

## Features:

- **ChatGPT functionality** is fully integrated into a lightweight desktop app.
- **No microphone support** in this release, designed to save system resources while maintaining core chat functionality.
- **Low system resource usage**, making it perfect for users with limited hardware resources.
- **Fast startup** and minimal overhead.

---

## Supported Distributions:

This release provides three formats for installation:

### 1. Debian-based distributions (.deb)

Supported examples:
- **Ubuntu**
- **Linux Mint**
- **Pop!_OS**
- **Debian**

To install:

```bash
sudo dpkg -i ChatGPT_1.0.0_amd64.deb
sudo apt -f install
🐧 Linux (.deb)
Download the .deb package.
Install using:
sudo dpkg -i ChatGPT_1.0.0_amd64.deb
sudo apt -f install
🐧 Linux (.rpm)
Download the .rpm package.
Install using:
sudo rpm -i ChatGPT-1.0.0-1.x86_64.rpm
🛠️ Development & Build Commands
If you want to contribute or build the project from source:
1. Installation
# Clone the repository
git clone https://github.com/emadjawad/chat-gpt-tauri.git

# Install dependencies
npm install
2. Development Mode
npm run tauri dev
3. Build for Production
# Build for Linux (AppImage, .deb, .rpm)
npm run tauri build

# Build for Windows (Setup & Portable)
npm run build:win
🧰 Tech Stack
Framework: Tauri + Rust for a lightweight desktop experience.
Frontend: Vite + TypeScript.
Styling: Basic, minimal styling with custom CSS.
Terminal Interaction: Not applicable as it's a GUI-based app.
👨‍💻 Author
Emad
Location: Istanbul, Turkey 🇹🇷
GitHub: @emadjawad
Built with passion for Linux users who want a simple, efficient ChatGPT experience without running a full browser.
License
MIT License – Feel free to use, contribute, and modify this app as you wish!
