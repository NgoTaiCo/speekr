use thiserror::Error;

use crate::language::Language;

pub trait TtsEngine {
    fn speak(&self, text: &str, language: Language) -> Result<(), TtsError>;
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
mod piper;
#[cfg(target_os = "windows")]
mod windows;

pub fn speak(text: &str, language: Language) -> Result<(), TtsError> {
    let text = text.trim();
    if text.is_empty() {
        return Err(TtsError::EmptyText);
    }

    if let Some(model_path) = piper::model_path(language) {
        match piper::PiperTts::new(model_path).speak(text, language) {
            Ok(()) => return Ok(()),
            Err(error) => eprintln!("speekr: Piper failed, falling back to OS TTS: {error}"),
        }
    }

    platform_speak(text, language)
}

#[cfg(target_os = "windows")]
fn platform_speak(text: &str, language: Language) -> Result<(), TtsError> {
    windows::WindowsTts.speak(text, language)
}

#[cfg(target_os = "macos")]
fn platform_speak(text: &str, language: Language) -> Result<(), TtsError> {
    macos::MacOsTts.speak(text, language)
}

#[cfg(target_os = "linux")]
fn platform_speak(text: &str, language: Language) -> Result<(), TtsError> {
    linux::LinuxTts.speak(text, language)
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
fn platform_speak(_: &str, _: Language) -> Result<(), TtsError> {
    Err(TtsError::UnsupportedOs)
}
