use thiserror::Error;
use tray_icon::{
    Icon, TrayIcon, TrayIconBuilder,
    menu::{CheckMenuItem, Menu, MenuId, MenuItem, PredefinedMenuItem},
};

use crate::language::{Gender, LanguageMode};

pub struct AppTray {
    _tray_icon: TrayIcon,
    quit_id: MenuId,
    cancel_id: MenuId,
    cancel_item: MenuItem,
    _quit_item: MenuItem,
    // voice gender
    voice_female_id: MenuId,
    voice_male_id: MenuId,
    voice_female_item: CheckMenuItem,
    voice_male_item: CheckMenuItem,
    // language mode
    lang_auto_id: MenuId,
    lang_english_id: MenuId,
    lang_vietnamese_id: MenuId,
    lang_auto_item: CheckMenuItem,
    lang_english_item: CheckMenuItem,
    lang_vietnamese_item: CheckMenuItem,
}

#[derive(Debug, Error)]
pub enum TrayError {
    #[error("tray icon error: {0}")]
    Tray(#[from] tray_icon::Error),
    #[error("menu error: {0}")]
    Menu(#[from] tray_icon::menu::Error),
    #[error("icon error: {0}")]
    Icon(#[from] tray_icon::BadIcon),
}

pub fn create() -> Result<AppTray, TrayError> {
    let menu = Menu::new();

    let voice_female_item = CheckMenuItem::new("Voice: Female", true, true, None);
    let voice_male_item = CheckMenuItem::new("Voice: Male", true, false, None);
    let sep1 = PredefinedMenuItem::separator();

    let lang_auto_item = CheckMenuItem::new("Lang: Auto-detect", true, true, None);
    let lang_english_item = CheckMenuItem::new("Lang: English", true, false, None);
    let lang_vietnamese_item = CheckMenuItem::new("Lang: Vietnamese", true, false, None);
    let sep2 = PredefinedMenuItem::separator();

    let cancel_item = MenuItem::new("Cancel current speech", false, None);
    let sep3 = PredefinedMenuItem::separator();

    let quit_item = MenuItem::new("Quit", true, None);

    menu.append(&voice_female_item)?;
    menu.append(&voice_male_item)?;
    menu.append(&sep1)?;
    menu.append(&lang_auto_item)?;
    menu.append(&lang_english_item)?;
    menu.append(&lang_vietnamese_item)?;
    menu.append(&sep2)?;
    menu.append(&cancel_item)?;
    menu.append(&sep3)?;
    menu.append(&quit_item)?;

    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_menu_on_left_click(true)
        .with_tooltip("speekr - Speak: Ctrl+Alt+T, Translate: Ctrl+Alt+G")
        .with_title("speekr")
        .with_icon(build_icon()?)
        .build()?;

    Ok(AppTray {
        _tray_icon: tray_icon,
        quit_id: quit_item.id().clone(),
        cancel_id: cancel_item.id().clone(),
        cancel_item,
        _quit_item: quit_item,
        voice_female_id: voice_female_item.id().clone(),
        voice_male_id: voice_male_item.id().clone(),
        voice_female_item,
        voice_male_item,
        lang_auto_id: lang_auto_item.id().clone(),
        lang_english_id: lang_english_item.id().clone(),
        lang_vietnamese_id: lang_vietnamese_item.id().clone(),
        lang_auto_item,
        lang_english_item,
        lang_vietnamese_item,
    })
}

impl AppTray {
    pub fn is_quit_event(&self, id: &MenuId) -> bool {
        id == &self.quit_id
    }

    pub fn is_cancel_event(&self, id: &MenuId) -> bool {
        id == &self.cancel_id
    }

    pub fn is_voice_female_event(&self, id: &MenuId) -> bool {
        id == &self.voice_female_id
    }

    pub fn is_voice_male_event(&self, id: &MenuId) -> bool {
        id == &self.voice_male_id
    }

    pub fn is_lang_auto_event(&self, id: &MenuId) -> bool {
        id == &self.lang_auto_id
    }

    pub fn is_lang_english_event(&self, id: &MenuId) -> bool {
        id == &self.lang_english_id
    }

    pub fn is_lang_vietnamese_event(&self, id: &MenuId) -> bool {
        id == &self.lang_vietnamese_id
    }

    pub fn set_cancel_enabled(&self, enabled: bool) {
        self.cancel_item.set_enabled(enabled);
    }

    pub fn set_voice(&self, gender: Gender) {
        self.voice_female_item.set_checked(gender == Gender::Female);
        self.voice_male_item.set_checked(gender == Gender::Male);
    }

    pub fn set_language_mode(&self, mode: LanguageMode) {
        self.lang_auto_item.set_checked(mode == LanguageMode::Auto);
        self.lang_english_item.set_checked(mode == LanguageMode::English);
        self.lang_vietnamese_item.set_checked(mode == LanguageMode::Vietnamese);
    }
}

fn build_icon() -> Result<Icon, tray_icon::BadIcon> {
    let width = 32;
    let height = 32;
    let mut rgba = vec![0_u8; width * height * 4];

    for y in 0..height {
        for x in 0..width {
            let index = (y * width + x) * 4;
            let mic_head = (11..=20).contains(&x) && (5..=18).contains(&y);
            let mic_body = (9..=22).contains(&x) && (13..=21).contains(&y);
            let stem = (15..=16).contains(&x) && (21..=27).contains(&y);
            let base = (10..=21).contains(&x) && (27..=29).contains(&y);

            if mic_head || mic_body || stem || base {
                rgba[index] = 36;
                rgba[index + 1] = 110;
                rgba[index + 2] = 185;
                rgba[index + 3] = 255;
            }

            let shine = (13..=14).contains(&x) && (7..=14).contains(&y);
            if shine {
                rgba[index] = 235;
                rgba[index + 1] = 245;
                rgba[index + 2] = 255;
                rgba[index + 3] = 220;
            }
        }
    }

    Icon::from_rgba(rgba, width as u32, height as u32)
}
