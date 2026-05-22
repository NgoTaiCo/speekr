use std::{
    process::Command,
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use arboard::Clipboard;
use thiserror::Error;

const COPY_WAIT: Duration = Duration::from_millis(800);
const COPY_POLL: Duration = Duration::from_millis(40);

#[derive(Debug, Error)]
pub enum ClipboardReadError {
    #[error("clipboard error: {0}")]
    Clipboard(#[from] arboard::Error),
    #[error("failed to trigger copy command: {0}")]
    CopyCommand(String),
    #[error("no selected text was copied")]
    EmptySelection,
}

pub fn read_selected_text() -> Result<String, ClipboardReadError> {
    let mut clipboard = Clipboard::new()?;
    let previous_text = clipboard.get_text().ok();
    let sentinel = sentinel_text();

    clipboard.set_text(sentinel.clone())?;
    trigger_copy()?;

    let deadline = Instant::now() + COPY_WAIT;
    let mut copied = None;

    while Instant::now() < deadline {
        if let Ok(text) = clipboard.get_text() {
            if text != sentinel {
                copied = Some(text);
                break;
            }
        }
        thread::sleep(COPY_POLL);
    }

    if let Some(previous_text) = previous_text {
        let _ = clipboard.set_text(previous_text);
    }

    let text = copied.unwrap_or_default();
    let text = text.trim().to_owned();
    if text.is_empty() {
        return Err(ClipboardReadError::EmptySelection);
    }

    Ok(text)
}

fn sentinel_text() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("__speekr_clipboard_probe_{now}__")
}

#[cfg(target_os = "windows")]
fn trigger_copy() -> Result<(), ClipboardReadError> {
    let status = hidden_command("powershell")
        .args([
            "-NoProfile",
            "-STA",
            "-WindowStyle",
            "Hidden",
            "-Command",
            "Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.SendKeys]::SendWait('^c')",
        ])
        .status()
        .map_err(|error| ClipboardReadError::CopyCommand(error.to_string()))?;

    if status.success() {
        Ok(())
    } else {
        Err(ClipboardReadError::CopyCommand(format!(
            "PowerShell SendKeys exited with {status}"
        )))
    }
}

#[cfg(target_os = "macos")]
fn trigger_copy() -> Result<(), ClipboardReadError> {
    let status = Command::new("osascript")
        .args([
            "-e",
            "tell application \"System Events\" to keystroke \"c\" using command down",
        ])
        .status()
        .map_err(|error| ClipboardReadError::CopyCommand(error.to_string()))?;

    if status.success() {
        Ok(())
    } else {
        Err(ClipboardReadError::CopyCommand(format!(
            "osascript copy exited with {status}"
        )))
    }
}

#[cfg(target_os = "linux")]
fn trigger_copy() -> Result<(), ClipboardReadError> {
    let status = Command::new("sh")
        .args([
            "-c",
            "if command -v xdotool >/dev/null 2>&1; then xdotool key --clearmodifiers ctrl+c; elif command -v wtype >/dev/null 2>&1; then wtype -M ctrl c -m ctrl; else exit 127; fi",
        ])
        .status()
        .map_err(|error| ClipboardReadError::CopyCommand(error.to_string()))?;

    if status.success() {
        Ok(())
    } else {
        Err(ClipboardReadError::CopyCommand(
            "install xdotool for X11 or wtype for Wayland to copy selected text".to_owned(),
        ))
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
fn trigger_copy() -> Result<(), ClipboardReadError> {
    Err(ClipboardReadError::CopyCommand(
        "automatic copy is not implemented for this OS".to_owned(),
    ))
}

#[cfg(target_os = "windows")]
fn hidden_command(program: &str) -> Command {
    use std::os::windows::process::CommandExt;

    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let mut command = Command::new(program);
    command.creation_flags(CREATE_NO_WINDOW);
    command
}
