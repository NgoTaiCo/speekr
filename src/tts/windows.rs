use std::{
    io::Read,
    io::Write,
    process::{Command, Stdio},
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};

use super::{TtsEngine, TtsError};
use crate::language::{Gender, Language};

pub struct WindowsTts;

impl TtsEngine for WindowsTts {
    fn speak(
        &self,
        text: &str,
        language: Language,
        gender: Gender,
        stop_signal: &AtomicBool,
    ) -> Result<(), TtsError> {
        let script = r#"
Add-Type -AssemblyName System.Speech
$speaker = New-Object System.Speech.Synthesis.SpeechSynthesizer
$speaker.SetOutputToDefaultAudioDevice()
$voiceHint = $env:SPEEKR_VOICE_HINT
if ($env:SPEEKR_VOICE) {
    $voiceHint = $env:SPEEKR_VOICE
}
$voiceSelected = $false

if ($voiceHint) {
    try {
        $speaker.SelectVoice($voiceHint)
        $voiceSelected = $true
    } catch {}
}

$installed = @($speaker.GetInstalledVoices() | ForEach-Object { $_.VoiceInfo })
$cultureName = $env:SPEEKR_CULTURE
if (-not $voiceSelected -and $cultureName) {
  try {
    $culture = [System.Globalization.CultureInfo]::GetCultureInfo($cultureName)
        $matches = @($installed | Where-Object { $_.Culture.Name -eq $culture.Name })
        if ($matches.Count -gt 0) {
            $preferred = $matches |
                Sort-Object -Property @{
                    Expression = {
                        if ($_.Name -match 'Natural|Neural') { 0 }
                        elseif ($_.Name -match 'Microsoft') { 1 }
                        else { 2 }
                    }
                }, Name |
                Select-Object -First 1
            if ($preferred) {
                $speaker.SelectVoice($preferred.Name)
                $voiceSelected = $true
            }
        }
  } catch {}
}

if (-not $voiceSelected) {
    try {
        $speaker.SelectVoiceByHints([System.Speech.Synthesis.VoiceGender]::NotSet)
    } catch {}
}

$text = [Console]::In.ReadToEnd()
$speaker.Speak($text)
"#;

        let mut child = hidden_command("powershell")
            .args(["-NoProfile", "-STA", "-Command", script])
            .env("SPEEKR_CULTURE", language.windows_culture())
            .env("SPEEKR_VOICE_HINT", language.windows_voice_hint(gender))
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes())?;
        }

        let Some(output) = wait_with_cancel(child, stop_signal)? else {
            return Ok(());
        };
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

fn wait_with_cancel(
    mut child: std::process::Child,
    stop_signal: &AtomicBool,
) -> Result<Option<std::process::Output>, TtsError> {
    loop {
        if stop_signal.load(Ordering::SeqCst) {
            let _ = child.kill();
            let _ = child.wait();
            return Ok(None);
        }

        if let Some(status) = child.try_wait()? {
            let mut stderr = Vec::new();
            if let Some(mut pipe) = child.stderr.take() {
                let _ = pipe.read_to_end(&mut stderr);
            }
            return Ok(Some(std::process::Output {
                status,
                stdout: Vec::new(),
                stderr,
            }));
        }

        thread::sleep(Duration::from_millis(50));
    }
}

fn hidden_command(program: &str) -> Command {
    use std::os::windows::process::CommandExt;

    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let mut command = Command::new(program);
    command.creation_flags(CREATE_NO_WINDOW);
    command
}
