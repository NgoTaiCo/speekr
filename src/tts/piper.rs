use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

use super::{TtsEngine, TtsError};
use crate::language::Language;

pub struct PiperTts {
    model_path: PathBuf,
}

impl PiperTts {
    pub fn new(model_path: PathBuf) -> Self {
        Self { model_path }
    }
}

impl TtsEngine for PiperTts {
    fn speak(&self, text: &str, _language: Language) -> Result<(), TtsError> {
        let wav_path = std::env::temp_dir().join(format!("speekr-piper-{}.wav", unique_id()));
        let mut child = Command::new(piper_command())
            .arg("--model")
            .arg(&self.model_path)
            .arg("--output_file")
            .arg(&wav_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes())?;
        }

        let output = child.wait_with_output()?;
        if !output.status.success() {
            return Err(TtsError::CommandFailed {
                engine: "Piper",
                status: output.status,
                stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
            });
        }

        play_wav(&wav_path)?;
        let _ = fs::remove_file(&wav_path);
        Ok(())
    }
}

pub fn model_path(language: Language) -> Option<PathBuf> {
    std::env::var_os(language.piper_env_key())
        .or_else(|| std::env::var_os("SPEEKR_PIPER_MODEL"))
        .map(PathBuf::from)
        .filter(|path| !path.as_os_str().is_empty())
}

fn piper_command() -> String {
    std::env::var("SPEEKR_PIPER_BIN").unwrap_or_else(|_| "piper".to_owned())
}

fn play_wav(path: &PathBuf) -> Result<(), TtsError> {
    let stream = rodio::OutputStreamBuilder::open_default_stream()
        .map_err(|error| TtsError::Audio(error.to_string()))?;
    let file = fs::File::open(path)?;
    let sink =
        rodio::play(stream.mixer(), file).map_err(|error| TtsError::Audio(error.to_string()))?;
    sink.sleep_until_end();
    Ok(())
}

fn unique_id() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
}
