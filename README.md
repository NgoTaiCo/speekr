# speekr

speekr la ung dung desktop nen Rust, chay background va doc toan bo van ban duoc boi den bang TTS local da nen tang (Windows, macOS, Linux).

Ung dung toi uu cho workflow nhanh:

1. Boi den text trong bat ky app nao.
2. Bam `Ctrl+Shift+T`.
3. speekr tu copy text dang chon.
4. Hien popup dich song ngu gan vi tri chuot.
5. Doc to text theo ngon ngu dau ra dang chon trong system tray.

## Tinh nang chinh

- Chay ngam voi icon tray, khong can mo cua so chinh.
- Hotkey global mac dinh: `Ctrl+Shift+T`.
- TTS local theo he dieu hanh (khong bat buoc cloud).
- Ho tro Piper de cai thien chat luong giong doc local.
- Dich offline bang Argos Translate (tuy chon).
- Chon ngon ngu output truc tiep tu tray menu.

## Nen tang va TTS engine

| OS | Engine mac dinh | Ghi chu |
| --- | --- | --- |
| Windows 10+ | PowerShell + `System.Speech.Synthesis` | Co san trong Windows PowerShell/.NET Framework |
| macOS 12+ | `say` | Co san trong macOS |
| Linux | `espeak-ng` (fallback `spd-say`) | Cai qua package manager |

Luu y:

- Chat luong giong doc phu thuoc voices co san tren may.
- Linux mac dinh dung voice theo ma ngon ngu (`vi`, `en`, ...). Co the override qua bien moi truong `SPEEKR_VOICE`.

## Kien truc tong quan

- `src/main.rs`: event loop, hotkey, flow chinh.
- `src/clipboard.rs`: copy text dang boi den tu app foreground.
- `src/translate.rs`: dich offline (Argos Translate).
- `src/popup.rs`: popup hien text goc + text dich.
- `src/tray.rs`: system tray menu + chon language.
- `src/tts/*`: TTS theo tung he dieu hanh + Piper.
- `src/language.rs`: danh sach ngon ngu va mapping voice/culture.

## Cai dat nhanh

### 1) Clone va build

```sh
git clone https://github.com/NgoTaiCo/speekr.git
cd speekr
cargo build --release
```

Binary sau khi build:

```text
target/release/speekr
target/release/speekr.exe
```

### 2) Chay ung dung

Windows:

```powershell
.\target\release\speekr.exe
```

macOS/Linux:

```sh
./target/release/speekr
```

## Cai dat theo he dieu hanh

### Windows

Yeu cau:

- Rust toolchain
- Windows 10 tro len

Build:

```powershell
cargo build --release
```

Ghi chu:

- Build release dung Windows GUI subsystem, nen khong giu console mo.
- Tray icon la UI chinh.

### macOS

Yeu cau:

- Rust toolchain
- macOS 12 tro len

Build:

```sh
cargo build --release
```

Ghi chu:

- He thong co the hoi quyen Accessibility vi app gui phim tat `Cmd+C`.

### Ubuntu 22+

Dependencies runtime/build:

```sh
sudo apt update
sudo apt install -y espeak-ng libgtk-3-dev libxdo-dev libayatana-appindicator3-dev libwebkit2gtk-4.1-dev xdotool
```

Neu distro khong co `libayatana-appindicator3-dev`, dung `libappindicator3-dev`.

Build:

```sh
cargo build --release
```

Ghi chu:

- `espeak-ng`: TTS runtime toi thieu.
- `libwebkit2gtk-4.1-dev`: bat buoc cho popup WebView.
- `xdotool`: copy text tren X11.
- Wayland: can `wtype` va quyen cho synthetic keyboard input.

### Fedora

```sh
sudo dnf install -y espeak-ng gtk3-devel libxdo-devel libappindicator-gtk3-devel webkit2gtk4.1-devel xdotool
cargo build --release
```

### Arch Linux

```sh
sudo pacman -S --needed espeak-ng gtk3 xdotool libappindicator-gtk3 webkit2gtk-4.1
cargo build --release
```

## Cau hinh bien moi truong

### TTS native

| Bien moi truong | Mo ta |
| --- | --- |
| `SPEEKR_VOICE` | Override voice cho Linux `espeak-ng` |
| `SPEEKR_CULTURE` | Culture cho Windows voice selection (duoc app set tu language da chon) |

### Piper (chat luong giong doc cao hon)

Dat model mac dinh:

```powershell
set SPEEKR_PIPER_MODEL=D:\models\piper\voice.onnx
set SPEEKR_PIPER_BIN=D:\tools\piper\piper.exe
```

Dat model theo ngon ngu:

```powershell
set SPEEKR_PIPER_MODEL_VI=D:\models\piper\vi.onnx
set SPEEKR_PIPER_MODEL_EN=D:\models\piper\en.onnx
set SPEEKR_PIPER_MODEL_JA=D:\models\piper\ja.onnx
```

Thu tu uu tien model:

1. `SPEEKR_PIPER_MODEL_<LANG>` (vi du: `SPEEKR_PIPER_MODEL_VI`)
2. `SPEEKR_PIPER_MODEL`
3. Fallback sang TTS native cua he dieu hanh

Neu khong cai Piper, speekr van chay binh thuong voi engine mac dinh cua OS.

## Dich offline voi Argos Translate

speekr khong bat buoc internet de dich neu da cai model Argos local.

```sh
pip install argostranslate
argospm update
argospm install translate-en_vi
```

Hien tai flow mac dinh la dich tu English (`en`) sang ngon ngu output da chon trong tray.

## Script build

- `scripts/build-windows.ps1`
- `scripts/build-macos.sh`
- `scripts/build-linux.sh`

Tat ca script hien tai deu build release:

```sh
cargo build --release
```

## Troubleshooting

### Bam hotkey nhung khong doc

- Kiem tra text co duoc boi den that khong.
- Tren Linux, dam bao co `xdotool` (X11) hoac `wtype` (Wayland).
- Kiem tra app co dang chay va tray icon co hien.

### Khong cogiong Viet chat luong cao

- Cai Piper va nap model tieng Viet qua `SPEEKR_PIPER_MODEL_VI`.
- Neu khong, app se dung voice he thong (co the chat luong thap hon).

### Popup khong hien

- Linux: kiem tra `webkit2gtk` va appindicator da cai dung.
- Kiem tra moi truong desktop co cho phep tao WebView/tray icon.

## Build voi feature tuy chon

`edge-audio` hien la placeholder cho fallback audio trong tuong lai:

```sh
cargo build --release --features edge-audio
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
        ├── piper.rs
        ├── windows.rs
        ├── macos.rs
        └── linux.rs
```

## Roadmap de xuat

- Cho phep custom hotkey trong tray settings.
- Them profile voice/toc do doc cho tung language.
- Tu dong update language packs cho dich offline.
- Dong goi installer cho Windows/macOS/Linux.
