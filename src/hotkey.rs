use global_hotkey::{
    GlobalHotKeyManager,
    hotkey::{Code, HotKey, Modifiers},
};
use thiserror::Error;

pub struct HotkeyRegistration {
    _manager: GlobalHotKeyManager,
    hotkey: HotKey,
}

impl HotkeyRegistration {
    pub fn id(&self) -> u32 {
        self.hotkey.id()
    }
}

#[derive(Debug, Error)]
pub enum HotkeyError {
    #[error("global hotkey error: {0}")]
    Global(#[from] global_hotkey::Error),
}

pub fn register_default() -> Result<HotkeyRegistration, HotkeyError> {
    let manager = GlobalHotKeyManager::new()?;
    let hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyT);
    manager.register(hotkey)?;

    Ok(HotkeyRegistration {
        _manager: manager,
        hotkey,
    })
}
