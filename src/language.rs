#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Vietnamese,
    English,
    Japanese,
    Korean,
    French,
    Spanish,
}

impl Language {
    pub const ALL: [Language; 6] = [
        Language::Vietnamese,
        Language::English,
        Language::Japanese,
        Language::Korean,
        Language::French,
        Language::Spanish,
    ];

    pub fn code(self) -> &'static str {
        match self {
            Language::Vietnamese => "vi",
            Language::English => "en",
            Language::Japanese => "ja",
            Language::Korean => "ko",
            Language::French => "fr",
            Language::Spanish => "es",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Language::Vietnamese => "Vietnamese",
            Language::English => "English",
            Language::Japanese => "Japanese",
            Language::Korean => "Korean",
            Language::French => "French",
            Language::Spanish => "Spanish",
        }
    }

    pub fn translation_source(self) -> &'static str {
        "en"
    }

    pub fn windows_culture(self) -> &'static str {
        match self {
            Language::Vietnamese => "vi-VN",
            Language::English => "en-US",
            Language::Japanese => "ja-JP",
            Language::Korean => "ko-KR",
            Language::French => "fr-FR",
            Language::Spanish => "es-ES",
        }
    }

    #[cfg(target_os = "macos")]
    pub fn macos_voice(self) -> Option<&'static str> {
        match self {
            Language::Vietnamese => None,
            Language::English => Some("Samantha"),
            Language::Japanese => Some("Kyoko"),
            Language::Korean => Some("Yuna"),
            Language::French => Some("Thomas"),
            Language::Spanish => Some("Monica"),
        }
    }

    #[cfg(target_os = "linux")]
    pub fn espeak_voice(self) -> &'static str {
        self.code()
    }

    pub fn piper_env_key(self) -> String {
        format!("SPEEKR_PIPER_MODEL_{}", self.code().to_uppercase())
    }
}
