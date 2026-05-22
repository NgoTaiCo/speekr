<div align="center">

# 🔊 speekr

**Neural text-to-speech and instant translation — right from your system tray.**

[![Release](https://github.com/NgoTaiCo/speekr/actions/workflows/release.yml/badge.svg)](https://github.com/NgoTaiCo/speekr/actions/workflows/release.yml)
[![GitHub release](https://img.shields.io/github/v/release/NgoTaiCo/speekr?style=flat-square&color=blue)](https://github.com/NgoTaiCo/speekr/releases)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey?style=flat-square)
![Built with Rust](https://img.shields.io/badge/built%20with-Rust-orange?style=flat-square&logo=rust)
[![License](https://img.shields.io/badge/license-Apache%202.0-green?style=flat-square)](LICENSE)

Select any text anywhere → press a hotkey → hear it spoken in a neural voice, or see it translated in a popup.

</div>

---

## ✨ Features

- 🎙️ **Neural TTS** — Microsoft Neural voices via `edge-tts` (Jenny, Guy, HoaiMy, NamMinh)
- 🌐 **Google Translate popup** — instant translation with a floating overlay
- 🚺🚹 **Voice gender** — switch Male / Female from the tray menu
- 🔤 **Language selection** — English / Vietnamese with auto-detect
- ⚡ **Zero UI** — lives entirely in your system tray, no windows
- ❌ **Cancellable** — re-press the hotkey or click *Cancel* in the tray to stop mid-speech

---

## 📋 Requirements

| Requirement | Purpose |
|-------------|---------|
| **Python 3** | Runs the `edge-tts` synthesis engine |
| **edge-tts** (`pip install edge-tts`) | Neural voice quality — falls back to OS TTS if missing |
| **Windows 10/11** *(WebView2)* | Pre-installed on Win10/11; required for the translate popup |

```sh
pip install edge-tts
```

---

## 📦 Installation

### Windows
Download **`speekr-setup-1.0.0.exe`** from [Releases](https://github.com/NgoTaiCo/speekr/releases) and run the installer.

> The installer offers optional **Desktop shortcut** and **Run on startup** options.

### macOS / Linux
Download the binary from [Releases](https://github.com/NgoTaiCo/speekr/releases):

```sh
# macOS
tar -xzf speekr-macos.tar.gz
./speekr

# Linux
tar -xzf speekr-linux.tar.gz
./speekr
```

---

## ⌨️ Usage

| Hotkey | Action |
|--------|--------|
| `Ctrl + Alt + T` | Read selected text aloud |
| `Ctrl + Alt + G` | Translate selected text (popup) |

**Tray menu** (right-click the 🔊 icon in the system tray):

```
✓ Voice: Female       ← toggle Male / Female
  Voice: Male
─────────────────────
✓ Lang: Auto-detect   ← toggle language mode
  Lang: English
  Lang: Vietnamese
─────────────────────
  Cancel current speech
─────────────────────
  Quit
```

---

## 🌍 Language Support

### Text-to-Speech

| Language | Female voice | Male voice |
|----------|-------------|------------|
| 🇺🇸 English | `en-US-JennyNeural` | `en-US-GuyNeural` |
| 🇻🇳 Vietnamese | `vi-VN-HoaiMyNeural` | `vi-VN-NamMinhNeural` |

> **Roadmap** — `edge-tts` supports ~80 languages (French, Japanese, Korean, Chinese, Spanish…). Additional languages will be added in future releases.

### Translation

Currently translates **to Vietnamese only**.

> **Roadmap** — Google Translate supports 130+ target languages. Selectable target language is planned.

### Auto-detect

> ⚠️ **Limitation** — Auto-detect currently distinguishes **English vs Vietnamese only** (based on Vietnamese-specific diacritics). Other languages will default to English voice. Proper multi-language detection is planned.

---

## 🏗️ Building from Source

```sh
git clone https://github.com/NgoTaiCo/speekr.git
cd speekr
cargo build --release
# binary at: target/release/speekr(.exe)
```

**Linux** — install system dependencies first:

```sh
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev libgtk-3-dev \
  libayatana-appindicator3-dev \
  libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
  libxkbcommon-dev pkg-config
```

---

## 📄 License

[Apache License 2.0](LICENSE)


- `Ctrl+Shift+T`: doc text da boi den bang TTS (chi English).
- `Ctrl+Shift+G`: dich doan van da boi den bang Google Translate va hien popup (khong phat audio).

## Scope hien tai

- TTS: chi ho tro English.
- Translation: flow rieng, khong di chung voi TTS.
- Cancel: bam lai `Ctrl+Shift+T` hoac chon menu `Cancel current speech` de dung doc ngay.

## TTS backend

speekr thu backend theo thu tu:

1. KittenTTS command (thu nghiem)
2. Fallback sang TTS he dieu hanh

### Bien moi truong TTS

| Bien moi truong | Mo ta |
| --- | --- |
| `SPEEKR_KITTEN_BIN` | Duong dan command KittenTTS (mac dinh: `kittentts`) |
| `SPEEKR_VOICE` | Override voice cho Linux `espeak-ng` va voice hint tren Windows |
| `SPEEKR_CULTURE` | Culture voice tren Windows (mac dinh app set `en-US`) |

## Translation (Google-like)

- Hotkey: `Ctrl+Shift+G`
- Co che: goi endpoint Google Translate de dich doan van.
- Translation chi hien popup, khong truyen qua TTS.

## Cai dat nhanh

```sh
git clone https://github.com/NgoTaiCo/speekr.git
cd speekr
cargo build --release
```

Run:

- Windows: `./target/release/speekr.exe`
- macOS/Linux: `./target/release/speekr`

## Build scripts

- `scripts/build-windows.ps1`
- `scripts/build-macos.sh`
- `scripts/build-linux.sh`

Tat ca script deu build release:

```sh
cargo build --release
```

## Cau truc du an

```text
speekr/
├── Cargo.toml
├── README.md
├── scripts/
│   ├── build-linux.sh
│   ├── build-macos.sh
│   └── build-windows.ps1
└── src/
    ├── main.rs
    ├── hotkey.rs
    ├── clipboard.rs
    ├── tray.rs
    ├── popup.rs
    ├── translate.rs
    ├── language.rs
    └── tts/
        ├── mod.rs
        ├── kitten.rs
        ├── windows.rs
        ├── macos.rs
        └── linux.rs
```
