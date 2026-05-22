#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    Vietnamese,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    Female,
    Male,
}

/// How the TTS language is determined for each request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LanguageMode {
    /// Auto-detect language from the selected text.
    Auto,
    English,
    Vietnamese,
}

impl Language {
    pub fn windows_culture(self) -> &'static str {
        match self {
            Language::English => "en-US",
            Language::Vietnamese => "vi-VN",
        }
    }

    pub fn windows_voice_hint(self, gender: Gender) -> &'static str {
        match (self, gender) {
            (Language::English, Gender::Female) => "Zira",
            (Language::English, Gender::Male) => "David",
            (Language::Vietnamese, _) => "",
        }
    }

    #[cfg(target_os = "macos")]
    pub fn macos_voice(self) -> Option<&'static str> {
        match self {
            Language::English => Some("Samantha"),
            Language::Vietnamese => None,
        }
    }

    #[cfg(target_os = "linux")]
    pub fn espeak_voice(self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Vietnamese => "vi",
        }
    }

    /// Voice name for edge-tts (Microsoft Neural voices via Python edge-tts package).
    pub fn edge_tts_voice(self, gender: Gender) -> &'static str {
        match (self, gender) {
            (Language::English, Gender::Female) => "en-US-JennyNeural",
            (Language::English, Gender::Male) => "en-US-GuyNeural",
            (Language::Vietnamese, Gender::Female) => "vi-VN-HoaiMyNeural",
            (Language::Vietnamese, Gender::Male) => "vi-VN-NamMinhNeural",
        }
    }

    /// Auto-detect language from text content.
    /// Returns Vietnamese if Vietnamese-specific characters are found, otherwise English.
    pub fn detect(text: &str) -> Language {
        let alpha_count = text.chars().filter(|c| c.is_alphabetic()).count();
        if alpha_count == 0 {
            return Language::English;
        }
        let viet_count = text.chars().filter(|c| is_vietnamese_char(*c)).count();
        // Threshold: if >8% of alphabetic characters are Vietnamese-specific → Vietnamese
        if viet_count * 12 >= alpha_count {
            Language::Vietnamese
        } else {
            Language::English
        }
    }
}

fn is_vietnamese_char(c: char) -> bool {
    matches!(
        c,
        'à' | 'á' | 'ả' | 'ã' | 'ạ'
            | 'ă' | 'ắ' | 'ặ' | 'ằ' | 'ẳ' | 'ẵ'
            | 'â' | 'ấ' | 'ậ' | 'ầ' | 'ẩ' | 'ẫ'
            | 'è' | 'é' | 'ẻ' | 'ẽ' | 'ẹ'
            | 'ê' | 'ế' | 'ệ' | 'ề' | 'ể' | 'ễ'
            | 'ì' | 'í' | 'ỉ' | 'ĩ' | 'ị'
            | 'ò' | 'ó' | 'ỏ' | 'õ' | 'ọ'
            | 'ô' | 'ố' | 'ộ' | 'ồ' | 'ổ' | 'ỗ'
            | 'ơ' | 'ớ' | 'ợ' | 'ờ' | 'ở' | 'ỡ'
            | 'ù' | 'ú' | 'ủ' | 'ũ' | 'ụ'
            | 'ư' | 'ứ' | 'ự' | 'ừ' | 'ử' | 'ữ'
            | 'ỳ' | 'ý' | 'ỷ' | 'ỹ' | 'ỵ'
            | 'đ'
            | 'À' | 'Á' | 'Ả' | 'Ã' | 'Ạ'
            | 'Ă' | 'Ắ' | 'Ặ' | 'Ằ' | 'Ẳ' | 'Ẵ'
            | 'Â' | 'Ấ' | 'Ậ' | 'Ầ' | 'Ẩ' | 'Ẫ'
            | 'È' | 'É' | 'Ẻ' | 'Ẽ' | 'Ẹ'
            | 'Ê' | 'Ế' | 'Ệ' | 'Ề' | 'Ể' | 'Ễ'
            | 'Ì' | 'Í' | 'Ỉ' | 'Ĩ' | 'Ị'
            | 'Ò' | 'Ó' | 'Ỏ' | 'Õ' | 'Ọ'
            | 'Ô' | 'Ố' | 'Ộ' | 'Ồ' | 'Ổ' | 'Ỗ'
            | 'Ơ' | 'Ớ' | 'Ợ' | 'Ờ' | 'Ở' | 'Ỡ'
            | 'Ù' | 'Ú' | 'Ủ' | 'Ũ' | 'Ụ'
            | 'Ư' | 'Ứ' | 'Ự' | 'Ừ' | 'Ử' | 'Ữ'
            | 'Ỳ' | 'Ý' | 'Ỷ' | 'Ỹ' | 'Ỵ'
            | 'Đ'
    )
}
