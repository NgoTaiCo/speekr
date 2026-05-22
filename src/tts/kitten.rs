use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::atomic::AtomicBool,
    time::{SystemTime, UNIX_EPOCH},
};

use super::{TtsEngine, TtsError};
use crate::language::Language;

pub struct KittenTts;

impl TtsEngine for KittenTts {
    fn speak(
        &self,
        text: &str,
        _language: Language,
        stop_signal: &AtomicBool,
    ) -> Result<(), TtsError> {
        if stop_signal.load(std::sync::atomic::Ordering::SeqCst) {
            return Ok(());
        }

        let wav_path = std::env::temp_dir().join(format!("speekr-kitten-{}.wav", unique_id()));

        let output = Command::new(kitten_command())
            .args(["--text", text, "--lang", "en", "--output", &wav_path.to_string_lossy()])
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output();

        let output = match output {
            Ok(output) => output,
            Err(error) => {
                safe_remove_temp_wav(&wav_path);
                return Err(error.into());
            }
        };

        if !output.status.success() {
            safe_remove_temp_wav(&wav_path);
            return Err(TtsError::CommandFailed {
                engine: "KittenTTS",
                status: output.status,
                stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
            });
        }

        if !wav_path.exists() {
            return Err(TtsError::CommandFailed {
                engine: "KittenTTS",
                status: output.status,
                stderr: "kitten command returned success but did not create wav output".to_owned(),
            });
        }

        let result = play_wav(&wav_path, stop_signal);
        safe_remove_temp_wav(&wav_path);
        result
    }
}

fn kitten_command() -> String {
    std::env::var("SPEEKR_KITTEN_BIN").unwrap_or_else(|_| "kittentts".to_owned())
}

fn play_wav(path: &PathBuf, stop_signal: &AtomicBool) -> Result<(), TtsError> {
    let stream = rodio::OutputStreamBuilder::open_default_stream()
        .map_err(|error| TtsError::Audio(error.to_string()))?;
    let file = fs::File::open(path)?;
    let sink =
        rodio::play(stream.mixer(), file).map_err(|error| TtsError::Audio(error.to_string()))?;

    while !sink.empty() {
        if stop_signal.load(std::sync::atomic::Ordering::SeqCst) {
            sink.stop();
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    Ok(())
}

fn safe_remove_temp_wav(path: &Path) {
    let temp_dir = std::env::temp_dir();
    let is_ours = path.starts_with(&temp_dir)
        && path
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.starts_with("speekr-kitten-") && name.ends_with(".wav"))
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
