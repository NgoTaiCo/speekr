#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
}

impl Language {
    pub fn windows_culture(self) -> &'static str {
        match self {
            Language::English => "en-US",
        }
    }

    pub fn windows_voice_hint(self) -> &'static str {
        match self {
            Language::English => "Jenny",
        }
    }

    #[cfg(target_os = "macos")]
    pub fn macos_voice(self) -> Option<&'static str> {
        match self {
            Language::English => Some("Samantha"),
        }
    }

    #[cfg(target_os = "linux")]
    pub fn espeak_voice(self) -> &'static str {
        match self {
            Language::English => "en",
        }
    }

    /// Voice name for edge-tts (Microsoft Neural voices via Python edge-tts package).
    pub fn edge_tts_voice(self) -> &'static str {
        match self {
            Language::English => "en-US-JennyNeural",
        }
    }
}
