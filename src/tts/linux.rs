use std::{
    io::Read,
    io::Write,
    process::{Command, Stdio},
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};

use super::{TtsEngine, TtsError};
use crate::language::Language;

pub struct LinuxTts;

impl TtsEngine for LinuxTts {
    fn speak(
        &self,
        text: &str,
        language: Language,
        stop_signal: &AtomicBool,
    ) -> Result<(), TtsError> {
        if command_exists("espeak-ng") {
            return speak_espeak(text, language, stop_signal);
        }

        if command_exists("spd-say") {
            return speak_spd_say(text, stop_signal);
        }

        Err(TtsError::CommandFailed {
            engine: "Linux TTS",
            status: exit_status(127),
            stderr: "install espeak-ng or speech-dispatcher".to_owned(),
        })
    }
}

fn speak_espeak(text: &str, language: Language, stop_signal: &AtomicBool) -> Result<(), TtsError> {
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

    let Some(output) = wait_with_cancel(child, stop_signal)? else {
        return Ok(());
    };
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

fn speak_spd_say(text: &str, stop_signal: &AtomicBool) -> Result<(), TtsError> {
    let child = Command::new("spd-say")
        .arg(text)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()?;

    let Some(output) = wait_with_cancel(child, stop_signal)? else {
        return Ok(());
    };

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

fn wait_with_cancel(
    mut child: std::process::Child,
    stop_signal: &AtomicBool,
) -> Result<Option<std::process::Output>, TtsError> {
    loop {
        if stop_signal.load(Ordering::SeqCst) {
            let _ = child.kill();
            let _ = child.wait();
            return Ok(None);
        }

        if let Some(status) = child.try_wait()? {
            let mut stderr = Vec::new();
            if let Some(mut pipe) = child.stderr.take() {
                let _ = pipe.read_to_end(&mut stderr);
            }
            return Ok(Some(std::process::Output {
                status,
                stdout: Vec::new(),
                stderr,
            }));
        }

        thread::sleep(Duration::from_millis(50));
    }

    unreachable!("loop always returns")
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
