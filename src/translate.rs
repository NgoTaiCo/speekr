use std::process::Command;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TranslateError {
    #[error("argos-translate is not installed or not on PATH")]
    MissingArgos,
    #[error("translation command failed: {0}")]
    Command(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub fn translate(text: &str, from: &str, to: &str) -> Result<String, TranslateError> {
    if from == to {
        return Ok(text.to_owned());
    }

    let output = Command::new("argos-translate")
        .args(["--from", from, "--to", to, text])
        .output()
        .map_err(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                TranslateError::MissingArgos
            } else {
                TranslateError::Io(error)
            }
        })?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        Err(TranslateError::Command(if stderr.is_empty() {
            output.status.to_string()
        } else {
            stderr
        }))
    }
}
