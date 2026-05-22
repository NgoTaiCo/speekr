# speekr

speekr la ung dung tray nen Rust voi 2 combo tach rieng:

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
