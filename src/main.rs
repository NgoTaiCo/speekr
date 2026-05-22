#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]

mod clipboard;
mod hotkey;
mod language;
mod popup;
mod translate;
mod tray;
mod tts;

use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
};

use global_hotkey::{GlobalHotKeyEvent, HotKeyState};
use tao::{
    event::Event,
    event_loop::{ControlFlow, EventLoopBuilder, EventLoopProxy},
};
use tray_icon::menu::MenuEvent;

#[derive(Debug)]
enum AppEvent {
    HotKey(GlobalHotKeyEvent),
    Menu(MenuEvent),
    ShowPopup(popup::PopupContent),
    ClosePopup,
}

fn main() {
    let mut event_loop_builder = EventLoopBuilder::<AppEvent>::with_user_event();
    let event_loop = event_loop_builder.build();

    let proxy = event_loop.create_proxy();
    GlobalHotKeyEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(AppEvent::HotKey(event));
    }));

    let proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(AppEvent::Menu(event));
    }));

    let hotkey = hotkey::register_default().expect("failed to register Ctrl+Shift+T");
    let tray = tray::create().expect("failed to create system tray icon");
    let mut selected_language = language::Language::Vietnamese;
    let mut popup = popup::TranslationPopup::default();
    let speaking = Arc::new(AtomicBool::new(false));
    let proxy = event_loop.create_proxy();

    event_loop.run(move |event, target, control_flow| {
        *control_flow = ControlFlow::Wait;

        let Event::UserEvent(event) = event else {
            return;
        };

        match event {
            AppEvent::HotKey(event)
                if event.id == hotkey.id() && event.state == HotKeyState::Released =>
            {
                let cursor = target.cursor_position().ok();
                speak_selection_once(
                    Arc::clone(&speaking),
                    proxy.clone(),
                    selected_language,
                    cursor.map(|position| (position.x, position.y)),
                );
            }
            AppEvent::HotKey(_) => {}
            AppEvent::Menu(event) if tray.select_language(event.id()).is_some() => {
                if let Some(language) = tray.select_language(event.id()) {
                    selected_language = language;
                    tray.set_selected_language(language);
                }
            }
            AppEvent::Menu(event) if tray.is_quit_event(event.id()) => {
                *control_flow = ControlFlow::Exit;
            }
            AppEvent::Menu(_) => {}
            AppEvent::ShowPopup(content) => {
                if let Err(error) = popup.show(target, proxy.clone(), content) {
                    eprintln!("speekr popup: {error}");
                }
            }
            AppEvent::ClosePopup => {
                popup.close();
            }
        }
    });
}

fn speak_selection_once(
    speaking: Arc<AtomicBool>,
    proxy: EventLoopProxy<AppEvent>,
    language: language::Language,
    cursor_position: Option<(f64, f64)>,
) {
    if speaking.swap(true, Ordering::SeqCst) {
        return;
    }

    thread::spawn(move || {
        if let Err(error) = speak_selected_text(proxy, language, cursor_position) {
            eprintln!("speekr: {error}");
        }
        speaking.store(false, Ordering::SeqCst);
    });
}

fn speak_selected_text(
    proxy: EventLoopProxy<AppEvent>,
    language: language::Language,
    cursor_position: Option<(f64, f64)>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let text = clipboard::read_selected_text()?;
    let translation = translate::translate(&text, language.translation_source(), language.code())
        .unwrap_or_else(|error| format!("Translation unavailable: {error}"));
    let spoken_text = if language.code() == language.translation_source()
        || translation.starts_with("Translation unavailable:")
    {
        text.clone()
    } else {
        translation.clone()
    };

    let _ = proxy.send_event(AppEvent::ShowPopup(popup::PopupContent {
        source_language: language.translation_source().to_owned(),
        target_language: language.code().to_owned(),
        original: text.clone(),
        translated: translation,
        cursor_position,
    }));

    tts::speak(&spoken_text, language)?;
    Ok(())
}
