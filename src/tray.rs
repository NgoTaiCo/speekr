use thiserror::Error;
use tray_icon::{
    Icon, TrayIcon, TrayIconBuilder,
    menu::{CheckMenuItem, Menu, MenuId, MenuItem, PredefinedMenuItem, Submenu},
};

use crate::language::Language;

pub struct AppTray {
    _tray_icon: TrayIcon,
    quit_id: MenuId,
    language_items: Vec<(Language, CheckMenuItem)>,
    _settings_item: MenuItem,
    _quit_item: MenuItem,
    _language_menu: Submenu,
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
    let settings_item = MenuItem::new("Settings (coming soon)", false, None);
    let quit_item = MenuItem::new("Quit", true, None);
    let separator = PredefinedMenuItem::separator();
    let separator2 = PredefinedMenuItem::separator();
    let language_menu = Submenu::new("Speaker output", true);
    let language_items = Language::ALL
        .iter()
        .copied()
        .map(|language| {
            let item = CheckMenuItem::new(
                format!("Read {}", language.label()),
                true,
                language == Language::Vietnamese,
                None,
            );
            (language, item)
        })
        .collect::<Vec<_>>();

    for (_, item) in &language_items {
        language_menu.append(item)?;
    }

    menu.append(&settings_item)?;
    menu.append(&separator)?;
    menu.append(&language_menu)?;
    menu.append(&separator2)?;
    menu.append(&quit_item)?;

    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_menu_on_left_click(true)
        .with_tooltip("speekr - Ctrl+Shift+T")
        .with_title("speekr")
        .with_icon(build_icon()?)
        .build()?;

    Ok(AppTray {
        _tray_icon: tray_icon,
        quit_id: quit_item.id().clone(),
        language_items,
        _settings_item: settings_item,
        _quit_item: quit_item,
        _language_menu: language_menu,
    })
}

impl AppTray {
    pub fn is_quit_event(&self, id: &MenuId) -> bool {
        id == &self.quit_id
    }

    pub fn select_language(&self, id: &MenuId) -> Option<Language> {
        self.language_items
            .iter()
            .find(|(_, item)| item.id() == id)
            .map(|(language, _)| *language)
    }

    pub fn set_selected_language(&self, selected: Language) {
        for (language, item) in &self.language_items {
            item.set_checked(*language == selected);
        }
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
