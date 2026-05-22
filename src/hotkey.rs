use global_hotkey::{
    GlobalHotKeyManager,
    hotkey::{Code, HotKey, Modifiers},
};
use thiserror::Error;

pub struct HotkeyRegistration {
    _manager: GlobalHotKeyManager,
    speak_hotkey: HotKey,
    translate_hotkey: HotKey,
}

impl HotkeyRegistration {
    pub fn speak_id(&self) -> u32 {
        self.speak_hotkey.id()
    }

    pub fn translate_id(&self) -> u32 {
        self.translate_hotkey.id()
    }
}

#[derive(Debug, Error)]
pub enum HotkeyError {
    #[error("global hotkey error: {0}")]
    Global(#[from] global_hotkey::Error),
}

pub fn register_default() -> Result<HotkeyRegistration, HotkeyError> {
    let manager = GlobalHotKeyManager::new()?;

    // Windows: Ctrl+Alt+T / Ctrl+Alt+G
    // macOS:   Cmd+Shift+T / Cmd+Shift+G  (Ctrl+Alt conflicts with Option combos)
    // Linux:   Ctrl+Alt+S / Ctrl+Alt+G    (Ctrl+Alt+T is reserved for opening a terminal)
    #[cfg(target_os = "windows")]
    let (speak_hotkey, translate_hotkey) = (
        HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyT),
        HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyG),
    );

    #[cfg(target_os = "macos")]
    let (speak_hotkey, translate_hotkey) = (
        HotKey::new(Some(Modifiers::META | Modifiers::SHIFT), Code::KeyT),
        HotKey::new(Some(Modifiers::META | Modifiers::SHIFT), Code::KeyG),
    );

    #[cfg(target_os = "linux")]
    let (speak_hotkey, translate_hotkey) = (
        HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyS),
        HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyG),
    );

    manager.register(speak_hotkey)?;
    manager.register(translate_hotkey)?;

    Ok(HotkeyRegistration {
        _manager: manager,
        speak_hotkey,
        translate_hotkey,
    })
}
