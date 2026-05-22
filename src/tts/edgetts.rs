use std::{
    fs,
    io::{Read, Write},
    path::PathBuf,
    process::{Command, Stdio},
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use super::{TtsEngine, TtsError};
use crate::language::{Gender, Language};

pub struct EdgeTts;

/// Inline Python: reads text from stdin (UTF-8), synthesises with edge-tts, saves MP3 to argv[2].
const SYNTH_SCRIPT: &str =
    "import sys,asyncio,edge_tts,io;\
sys.stdin=io.TextIOWrapper(sys.stdin.buffer,encoding='utf-8');\
asyncio.run(edge_tts.Communicate(sys.stdin.read().strip(),sys.argv[1]).save(sys.argv[2]))";

impl TtsEngine for EdgeTts {
    fn speak(
        &self,
        text: &str,
        language: Language,
        gender: Gender,
        stop_signal: &AtomicBool,
    ) -> Result<(), TtsError> {
        if stop_signal.load(Ordering::SeqCst) {
            return Ok(());
        }

        let mp3_path = std::env::temp_dir().join(format!("speekr-edge-{}.mp3", unique_id()));
        let voice = language.edge_tts_voice(gender);

        let mut child = new_command("python")
            .args(["-c", SYNTH_SCRIPT, voice, &mp3_path.to_string_lossy()])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(text.as_bytes());
        }

        // Wait for synthesis, support cancel during the network request
        let Some(output) = wait_with_cancel(child, stop_signal)? else {
            safe_remove(&mp3_path);
            return Ok(());
        };

        if !output.status.success() {
            safe_remove(&mp3_path);
            return Err(TtsError::CommandFailed {
                engine: "edge-tts",
                status: output.status,
                stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
            });
        }

        let result = play_audio(&mp3_path, stop_signal);
        safe_remove(&mp3_path);
        result
    }
}

fn unique_id() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
}

fn safe_remove(path: &PathBuf) {
    let _ = fs::remove_file(path);
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

fn play_audio(path: &PathBuf, stop_signal: &AtomicBool) -> Result<(), TtsError> {
    let stream = rodio::OutputStreamBuilder::open_default_stream()
        .map_err(|error| TtsError::Audio(error.to_string()))?;
    let file = fs::File::open(path)?;
    let sink =
        rodio::play(stream.mixer(), file).map_err(|error| TtsError::Audio(error.to_string()))?;

    while !sink.empty() {
        if stop_signal.load(Ordering::SeqCst) {
            sink.stop();
            break;
        }
        thread::sleep(Duration::from_millis(50));
    }
    Ok(())
}

/// Spawns a command without a visible console window on Windows.
#[cfg(target_os = "windows")]
fn new_command(program: &str) -> Command {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    let mut cmd = Command::new(program);
    cmd.creation_flags(CREATE_NO_WINDOW);
    cmd
}

#[cfg(not(target_os = "windows"))]
fn new_command(program: &str) -> Command {
    Command::new(program)
}
