use std::{
    fs,
    io::Read,
    process::{Command, Stdio},
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
    time::{SystemTime, UNIX_EPOCH},
};

use super::{TtsEngine, TtsError};
use crate::language::{Gender, Language};

pub struct MacOsTts;

impl TtsEngine for MacOsTts {
    fn speak(
        &self,
        text: &str,
        language: Language,
        _gender: Gender,
        stop_signal: &AtomicBool,
    ) -> Result<(), TtsError> {
        let path = std::env::temp_dir().join(format!("speekr-{}.txt", unique_id()));
        fs::write(&path, text)?;

        let mut command = Command::new("say");
        if let Some(voice) = language.macos_voice() {
            command.args(["-v", voice]);
        }

        let mut child = match command
            .arg("-f")
            .arg(&path)
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(child) => child,
            Err(error) => {
                safe_remove_temp_text(&path);
                return Err(error.into());
            }
        };

        let output = loop {
            if stop_signal.load(Ordering::SeqCst) {
                let _ = child.kill();
                let _ = child.wait();
                safe_remove_temp_text(&path);
                return Ok(());
            }

            if let Some(status) = child.try_wait()? {
                let mut stderr = Vec::new();
                if let Some(mut pipe) = child.stderr.take() {
                    let _ = pipe.read_to_end(&mut stderr);
                }
                break std::process::Output {
                    status,
                    stdout: Vec::new(),
                    stderr,
                };
            }

            thread::sleep(Duration::from_millis(50));
        };
        safe_remove_temp_text(&path);
        if output.status.success() {
            Ok(())
        } else {
            Err(TtsError::CommandFailed {
                engine: "macOS say",
                status: output.status,
                stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
            })
        }
    }
}

fn safe_remove_temp_text(path: &std::path::Path) {
    let temp_dir = std::env::temp_dir();
    let is_ours = path.starts_with(&temp_dir)
        && path
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.starts_with("speekr-") && name.ends_with(".txt"))
            .unwrap_or(false);

    if is_ours {
        let _ = fs::remove_file(path);
    }
}

fn unique_id() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
}
