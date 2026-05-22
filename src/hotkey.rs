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
    let speak_hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyT);
    let translate_hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyG);
    manager.register(speak_hotkey)?;
    manager.register(translate_hotkey)?;

    Ok(HotkeyRegistration {
        _manager: manager,
        speak_hotkey,
        translate_hotkey,
    })
}
