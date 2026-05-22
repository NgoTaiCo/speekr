use thiserror::Error;

#[derive(Debug, Error)]
pub enum TranslateError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("invalid translation response")]
    InvalidResponse,
}

pub fn translate_google_paragraph(text: &str, target_language: &str) -> Result<String, TranslateError> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Ok(String::new());
    }

    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36")
        .build()?;

    let response = client
        .get("https://translate.googleapis.com/translate_a/single")
        .query(&[
            ("client", "gtx"),
            ("sl", "auto"),
            ("tl", target_language),
            ("dt", "t"),
            ("q", trimmed),
        ])
        .send()?
        .error_for_status()?;

    let value: serde_json::Value = response.json()?;
    let segments = value
        .get(0)
        .and_then(|first| first.as_array())
        .ok_or(TranslateError::InvalidResponse)?;

    let mut output = String::new();
    for segment in segments {
        if let Some(piece) = segment.get(0).and_then(|entry| entry.as_str()) {
            output.push_str(piece);
        }
    }

    if output.is_empty() {
        return Err(TranslateError::InvalidResponse);
    }

    Ok(output)
}
