use std::sync::atomic::AtomicBool;

use thiserror::Error;

use crate::language::{Gender, Language};

pub trait TtsEngine {
    fn speak(
        &self,
        text: &str,
        language: Language,
        gender: Gender,
        stop_signal: &AtomicBool,
    ) -> Result<(), TtsError>;
}

#[derive(Debug, Error)]
pub enum TtsError {
    #[error("text is empty")]
    EmptyText,
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{engine} exited with {status}: {stderr}")]
    CommandFailed {
        engine: &'static str,
        status: std::process::ExitStatus,
        stderr: String,
    },
    #[error("audio playback error: {0}")]
    Audio(String),
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    #[error("TTS is not implemented for this OS")]
    UnsupportedOs,
}

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
mod edgetts;
mod kitten;
#[cfg(target_os = "windows")]
mod windows;

pub fn speak(text: &str, language: Language, gender: Gender, stop_signal: &AtomicBool) -> Result<(), TtsError> {
    let text = text.trim();
    if text.is_empty() {
        return Err(TtsError::EmptyText);
    }

    if let Err(error) = kitten::KittenTts.speak(text, language, gender, stop_signal) {
        eprintln!("speekr: KittenTTS failed, trying edge-tts: {error}");
    } else {
        return Ok(());
    }

    if stop_signal.load(std::sync::atomic::Ordering::SeqCst) {
        return Ok(());
    }

    if let Err(error) = edgetts::EdgeTts.speak(text, language, gender, stop_signal) {
        eprintln!("speekr: edge-tts failed, falling back to OS TTS: {error}");
    } else {
        return Ok(());
    }

    if stop_signal.load(std::sync::atomic::Ordering::SeqCst) {
        return Ok(());
    }

    platform_speak(text, language, gender, stop_signal)
}

#[cfg(target_os = "windows")]
fn platform_speak(
    text: &str,
    language: Language,
    gender: Gender,
    stop_signal: &AtomicBool,
) -> Result<(), TtsError> {
    windows::WindowsTts.speak(text, language, gender, stop_signal)
}

#[cfg(target_os = "macos")]
fn platform_speak(
    text: &str,
    language: Language,
    gender: Gender,
    stop_signal: &AtomicBool,
) -> Result<(), TtsError> {
    macos::MacOsTts.speak(text, language, gender, stop_signal)
}

#[cfg(target_os = "linux")]
fn platform_speak(
    text: &str,
    language: Language,
    gender: Gender,
    stop_signal: &AtomicBool,
) -> Result<(), TtsError> {
    linux::LinuxTts.speak(text, language, gender, stop_signal)
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
fn platform_speak(_: &str, _: Language, _: Gender, _: &AtomicBool) -> Result<(), TtsError> {
    Err(TtsError::UnsupportedOs)
}
