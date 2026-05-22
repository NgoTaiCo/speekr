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
        Arc, Mutex,
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
    SpeechFinished,
}

struct AppSettings {
    language_mode: language::LanguageMode,
    gender: language::Gender,
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

    let hotkey = hotkey::register_default().expect("failed to register global hotkeys");
    let tray = tray::create().expect("failed to create system tray icon");
    tray.set_cancel_enabled(false);
    let settings = Arc::new(Mutex::new(AppSettings {
        language_mode: language::LanguageMode::Auto,
        gender: language::Gender::Female,
    }));
    let mut popup = popup::TranslationPopup::default();
    let current_job = Arc::new(Mutex::new(None::<Arc<AtomicBool>>));
    let proxy = event_loop.create_proxy();

    event_loop.run(move |event, target, control_flow| {
        *control_flow = ControlFlow::Wait;

        let Event::UserEvent(event) = event else {
            return;
        };

        match event {
            AppEvent::HotKey(event)
                if event.id == hotkey.speak_id() && event.state == HotKeyState::Released =>
            {
                handle_hotkey(
                    Arc::clone(&current_job),
                    proxy.clone(),
                    &tray,
                    Arc::clone(&settings),
                );
            }
            AppEvent::HotKey(event)
                if event.id == hotkey.translate_id() && event.state == HotKeyState::Released =>
            {
                let cursor = target.cursor_position().ok();
                trigger_translation_popup(
                    proxy.clone(),
                    cursor.map(|position| (position.x, position.y)),
                );
            }
            AppEvent::HotKey(_) => {}
            AppEvent::Menu(event) if tray.is_quit_event(event.id()) => {
                *control_flow = ControlFlow::Exit;
            }
            AppEvent::Menu(event) if tray.is_cancel_event(event.id()) => {
                request_cancel(Arc::clone(&current_job), proxy.clone(), &tray);
            }
            AppEvent::Menu(event) if tray.is_voice_female_event(event.id()) => {
                settings.lock().expect("settings poisoned").gender = language::Gender::Female;
                tray.set_voice(language::Gender::Female);
            }
            AppEvent::Menu(event) if tray.is_voice_male_event(event.id()) => {
                settings.lock().expect("settings poisoned").gender = language::Gender::Male;
                tray.set_voice(language::Gender::Male);
            }
            AppEvent::Menu(event) if tray.is_lang_auto_event(event.id()) => {
                settings.lock().expect("settings poisoned").language_mode = language::LanguageMode::Auto;
                tray.set_language_mode(language::LanguageMode::Auto);
            }
            AppEvent::Menu(event) if tray.is_lang_english_event(event.id()) => {
                settings.lock().expect("settings poisoned").language_mode = language::LanguageMode::English;
                tray.set_language_mode(language::LanguageMode::English);
            }
            AppEvent::Menu(event) if tray.is_lang_vietnamese_event(event.id()) => {
                settings.lock().expect("settings poisoned").language_mode = language::LanguageMode::Vietnamese;
                tray.set_language_mode(language::LanguageMode::Vietnamese);
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
            AppEvent::SpeechFinished => {
                tray.set_cancel_enabled(false);
            }
        }
    });
}

fn handle_hotkey(
    current_job: Arc<Mutex<Option<Arc<AtomicBool>>>>,
    proxy: EventLoopProxy<AppEvent>,
    tray: &tray::AppTray,
    settings: Arc<Mutex<AppSettings>>,
) {
    let (language_mode, gender) = {
        let s = settings.lock().expect("settings poisoned");
        (s.language_mode, s.gender)
    };
    let maybe_stop_signal = {
        let mut job = current_job.lock().expect("current_job poisoned");
        if let Some(stop_signal) = job.take() {
            stop_signal.store(true, Ordering::SeqCst);
            None
        } else {
            let stop_signal = Arc::new(AtomicBool::new(false));
            *job = Some(Arc::clone(&stop_signal));
            Some(stop_signal)
        }
    };

    let Some(stop_signal) = maybe_stop_signal else {
        tray.set_cancel_enabled(false);
        let _ = proxy.send_event(AppEvent::ClosePopup);
        return;
    };

    tray.set_cancel_enabled(true);
    // Close any translation popup that may still be open from a Ctrl+Shift+G press
    let _ = proxy.send_event(AppEvent::ClosePopup);

    thread::spawn(move || {
        if let Err(error) = speak_selected_text(language_mode, gender, Arc::clone(&stop_signal)) {
            eprintln!("speekr: {error}");
        }

        let _ = proxy.send_event(AppEvent::ClosePopup);
        let mut job = current_job.lock().expect("current_job poisoned");
        if job
            .as_ref()
            .map(|active| Arc::ptr_eq(active, &stop_signal))
            .unwrap_or(false)
        {
            *job = None;
        }

        let _ = proxy.send_event(AppEvent::SpeechFinished);
    });
}

fn request_cancel(
    current_job: Arc<Mutex<Option<Arc<AtomicBool>>>>,
    proxy: EventLoopProxy<AppEvent>,
    tray: &tray::AppTray,
) {
    let cancelled = {
        let mut job = current_job.lock().expect("current_job poisoned");
        if let Some(stop_signal) = job.take() {
            stop_signal.store(true, Ordering::SeqCst);
            true
        } else {
            false
        }
    };

    if cancelled {
        tray.set_cancel_enabled(false);
        let _ = proxy.send_event(AppEvent::ClosePopup);
    }
}

fn speak_selected_text(
    language_mode: language::LanguageMode,
    gender: language::Gender,
    stop_signal: Arc<AtomicBool>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let text = clipboard::read_selected_text()?;
    if !stop_signal.load(Ordering::SeqCst) {
        let language = match language_mode {
            language::LanguageMode::Auto => language::Language::detect(&text),
            language::LanguageMode::English => language::Language::English,
            language::LanguageMode::Vietnamese => language::Language::Vietnamese,
        };
        tts::speak(&text, language, gender, &stop_signal)?;
    }
    Ok(())
}

fn trigger_translation_popup(proxy: EventLoopProxy<AppEvent>, cursor_position: Option<(f64, f64)>) {
    thread::spawn(move || {
        let original = match clipboard::read_selected_text() {
            Ok(text) => text,
            Err(error) => {
                let _ = proxy.send_event(AppEvent::ShowPopup(popup::PopupContent {
                    source_language: "auto".to_owned(),
                    target_language: "vi".to_owned(),
                    original: String::new(),
                    translated: format!("Cannot read selected text: {error}"),
                    cursor_position,
                }));
                return;
            }
        };

        let translated = match translate::translate_google_paragraph(&original, "vi") {
            Ok(value) => value,
            Err(error) => format!("Google translation unavailable: {error}"),
        };

        let _ = proxy.send_event(AppEvent::ShowPopup(popup::PopupContent {
            source_language: "auto".to_owned(),
            target_language: "vi".to_owned(),
            original,
            translated,
            cursor_position,
        }));
    });
}
