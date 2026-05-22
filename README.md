<div align="center">

# speekr 😄

**Too lazy to read? Or just want a blazing-fast audiobook experience?**<br>
**Download. Install. Highlight text. Press a key. Done. It's free. You're welcome. 😎**

[![Release](https://github.com/NgoTaiCo/speekr/actions/workflows/release.yml/badge.svg)](https://github.com/NgoTaiCo/speekr/actions/workflows/release.yml)
[![GitHub release](https://img.shields.io/github/v/release/NgoTaiCo/speekr?style=flat-square&color=blue)](https://github.com/NgoTaiCo/speekr/releases)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey?style=flat-square)
![Built with Rust](https://img.shields.io/badge/built%20with-Rust-orange?style=flat-square&logo=rust)
[![License](https://img.shields.io/badge/license-Apache%202.0-green?style=flat-square)](LICENSE)

</div>

---

## What does it do? 🤔

Highlight any text on your screen, hit a hotkey, and a neural voice reads it out loud. That's it. No app to open, no window to deal with — it just hides in your system tray and minds its own business. Oh, and it can translate stuff too. Because why not.

---

## Features 😏

| Feature | What's the deal |
|---------|-----------------|
| **Neural TTS** | Microsoft Neural voices via `edge-tts` — sounds way better than your OS robot voice |
| **Google Translate popup** | Floating translation overlay. Highlight → hotkey → boom, translated |
| **Male / Female voice** | Pick your narrator, no judgment |
| **English / Vietnamese** | Full neural voice support for both, with auto-detect so you don't have to think |
| **Zero UI** | Lives in the system tray. No windows, no distractions, no bloat |
| **Cancel anytime** | Made a mistake? Press the hotkey again or hit *Cancel* in the tray 😅 |

---

## Requirements 😬

Alright, let's be honest — there's a catch. Two, actually.

**speekr** is a Rust app that wraps [edge-tts](https://github.com/rany2/edge-tts), a Python library that calls Microsoft Edge's neural TTS API. So:

1. **You need Python 3** — speekr spawns a Python process to run `edge-tts` under the hood. No Python = no neural voice (it'll fall back to your OS's built-in robot voice, which, yeah).
2. **You need internet** — `edge-tts` streams audio from Microsoft's servers in real time. Offline = silent. No exceptions.

| Thing you need | Why |
|----------------|-----|
| **Python 3** | speekr wraps it to call edge-tts |
| **edge-tts** (`pip install edge-tts`) | Makes the actual API call to Microsoft's neural TTS |
| **Internet connection** | edge-tts calls Microsoft's servers — no connection, no voice |
| **Windows 10/11** | WebView2 pre-installed — needed only for the translate popup |

```sh
pip install edge-tts
```

---

## Installation 🙃

### Windows
Grab **`speekr-setup-1.0.0.exe`** from [Releases](https://github.com/NgoTaiCo/speekr/releases) and run it.

The installer lets you optionally add a **Desktop shortcut** and **Run on startup** — both off by default, because we respect your desktop real estate.

> **SmartScreen warning?** 😅 That's Windows being dramatic because the app isn't code-signed yet (certs cost money, this app is free). Click **"More info" → "Run anyway"** to proceed. The source code is fully open — feel free to audit it before running.

### macOS / Linux
Grab the binary from [Releases](https://github.com/NgoTaiCo/speekr/releases):

```sh
# macOS
tar -xzf speekr-macos.tar.gz
./speekr

# Linux
tar -xzf speekr-linux.tar.gz
./speekr
```

---

## How to use it 😇

Highlight text anywhere → press the hotkey → profit.

| Platform | Speak | Translate |
|----------|-------|-----------|
| Windows | `Ctrl + Alt + T` | `Ctrl + Alt + G` |
| macOS | `Cmd + Shift + T` | `Cmd + Shift + G` |
| Linux | `Ctrl + Alt + S` | `Ctrl + Alt + G` |

**Tray menu** — right-click the tray icon if you want to change stuff:

```
✓ Voice: Female       ← your narrator gender preference
  Voice: Male
─────────────────────
✓ Lang: Auto-detect   ← let the app figure it out
  Lang: English
  Lang: Vietnamese
─────────────────────
  Cancel current speech
─────────────────────
  Quit
```

---

## Language Support 😤

### Text-to-Speech

| Language | Female voice | Male voice |
|----------|-------------|------------|
| English | `en-US-JennyNeural` | `en-US-GuyNeural` |
| Vietnamese | `vi-VN-HoaiMyNeural` | `vi-VN-NamMinhNeural` |

> **Roadmap** — `edge-tts` supports ~80 languages. French, Japanese, Korean, Chinese, Spanish and more are coming. Eventually. 😂

### Translation

Currently translates **to Vietnamese only**.

> **Roadmap** — Selectable target language is planned. For now, Vietnamese it is.

### Auto-detect

> **Heads up** — Auto-detect only knows English vs Vietnamese right now (it checks for Vietnamese-specific diacritics). If you highlight French or Italian, it'll read it with an English accent and feel zero shame about it. Proper multi-language detection is on the roadmap. 🥲

---

## Build from Source 🤓

```sh
git clone https://github.com/NgoTaiCo/speekr.git
cd speekr
cargo build --release
# output: target/release/speekr(.exe)
```

**Linux** — grab these system deps first:

```sh
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev libgtk-3-dev \
  libayatana-appindicator3-dev \
  libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
  libxkbcommon-dev pkg-config
```

---

## License

[Apache License 2.0](LICENSE) — free as in free beer. 😄
