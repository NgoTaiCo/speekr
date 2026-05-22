use std::{
    io::Write,
    process::{Command, Stdio},
};

use super::{TtsEngine, TtsError};
use crate::language::Language;

pub struct WindowsTts;

impl TtsEngine for WindowsTts {
    fn speak(&self, text: &str, language: Language) -> Result<(), TtsError> {
        let script = r#"
Add-Type -AssemblyName System.Speech
$speaker = New-Object System.Speech.Synthesis.SpeechSynthesizer
$speaker.SetOutputToDefaultAudioDevice()
$cultureName = $env:SPEEKR_CULTURE
if ($cultureName) {
  try {
    $culture = [System.Globalization.CultureInfo]::GetCultureInfo($cultureName)
    $speaker.SelectVoiceByHints(
      [System.Speech.Synthesis.VoiceGender]::NotSet,
      [System.Speech.Synthesis.VoiceAge]::NotSet,
      0,
      $culture
    )
  } catch {}
}
$text = [Console]::In.ReadToEnd()
$speaker.Speak($text)
"#;

        let mut child = hidden_command("powershell")
            .args(["-NoProfile", "-STA", "-Command", script])
            .env("SPEEKR_CULTURE", language.windows_culture())
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes())?;
        }

        let output = child.wait_with_output()?;
        if output.status.success() {
            Ok(())
        } else {
            Err(TtsError::CommandFailed {
                engine: "Windows System.Speech",
                status: output.status,
                stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
            })
        }
    }
}

fn hidden_command(program: &str) -> Command {
    use std::os::windows::process::CommandExt;

    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let mut command = Command::new(program);
    command.creation_flags(CREATE_NO_WINDOW);
    command
}
