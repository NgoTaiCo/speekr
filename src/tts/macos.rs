use std::{
    fs,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use super::{TtsEngine, TtsError};
use crate::language::Language;

pub struct MacOsTts;

impl TtsEngine for MacOsTts {
    fn speak(&self, text: &str, language: Language) -> Result<(), TtsError> {
        let path = std::env::temp_dir().join(format!("speekr-{}.txt", unique_id()));
        fs::write(&path, text)?;

        let mut command = Command::new("say");
        if let Some(voice) = language.macos_voice() {
            command.args(["-v", voice]);
        }

        let output = command
            .arg("-f")
            .arg(&path)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::piped())
            .output();

        let _ = fs::remove_file(&path);

        let output = output?;
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

fn unique_id() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
}
