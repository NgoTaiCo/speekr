use std::{
    io::Write,
    process::{Command, Stdio},
};

use super::{TtsEngine, TtsError};
use crate::language::Language;

pub struct LinuxTts;

impl TtsEngine for LinuxTts {
    fn speak(&self, text: &str, language: Language) -> Result<(), TtsError> {
        if command_exists("espeak-ng") {
            return speak_espeak(text, language);
        }

        if command_exists("spd-say") {
            return speak_spd_say(text);
        }

        Err(TtsError::CommandFailed {
            engine: "Linux TTS",
            status: exit_status(127),
            stderr: "install espeak-ng or speech-dispatcher".to_owned(),
        })
    }
}

fn speak_espeak(text: &str, language: Language) -> Result<(), TtsError> {
    let voice =
        std::env::var("SPEEKR_VOICE").unwrap_or_else(|_| language.espeak_voice().to_owned());
    let mut child = Command::new("espeak-ng")
        .args(["-v", &voice, "--stdin"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(text.as_bytes())?;
    }

    let output = child.wait_with_output()?;
    if output.status.success() {
        Ok(())
    } else {
        Err(TtsError::CommandFailed {
            engine: "espeak-ng",
            status: output.status,
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        })
    }
}

fn speak_spd_say(text: &str) -> Result<(), TtsError> {
    let output = Command::new("spd-say")
        .arg(text)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(TtsError::CommandFailed {
            engine: "spd-say",
            status: output.status,
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        })
    }
}

fn command_exists(command: &str) -> bool {
    Command::new("sh")
        .args(["-c", &format!("command -v {command} >/dev/null 2>&1")])
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

#[cfg(unix)]
fn exit_status(code: i32) -> std::process::ExitStatus {
    use std::os::unix::process::ExitStatusExt;
    std::process::ExitStatus::from_raw(code << 8)
}

#[cfg(not(unix))]
fn exit_status(_: i32) -> std::process::ExitStatus {
    unreachable!("linux module is compiled only on unix targets")
}
