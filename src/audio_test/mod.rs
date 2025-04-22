use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use once_cell::sync::Lazy;

static AUDIO_ACTIVE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
static AUDIO_MESSAGE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

pub fn check_audio_test_active() -> bool {
    *AUDIO_ACTIVE.lock().unwrap()
}

pub fn exit_audio_test() {
    *AUDIO_ACTIVE.lock().unwrap() = false;
    *AUDIO_MESSAGE.lock().unwrap() = String::new();
}

pub fn enter_audio_test() {
    *AUDIO_ACTIVE.lock().unwrap() = true;
    *AUDIO_MESSAGE.lock().unwrap() = "Testing frequency range...".to_string();

    thread::spawn(|| {
        if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
            let sink = Sink::try_new(&stream_handle).unwrap();

            // You should generate or provide test tones across frequency range.
            // Here we simulate with one tone for brevity.
            let tones = vec![
                "assets/audio/100Hz.wav",
                "assets/audio/500Hz.wav",
                "assets/audio/1000Hz.wav",
                "assets/audio/5000Hz.wav",
                "assets/audio/10000Hz.wav",
            ];

            for tone_path in tones {
                if let Ok(file) = File::open(tone_path) {
                    let source = Decoder::new(BufReader::new(file)).unwrap();
                    sink.append(source);
                    sink.sleep_until_end();
                } else {
                    *AUDIO_MESSAGE.lock().unwrap() = format!("Missing tone file: {}", tone_path);
                    break;
                }
                thread::sleep(Duration::from_millis(300));
            }

            *AUDIO_MESSAGE.lock().unwrap() = "Audio test complete.".to_string();
        } else {
            *AUDIO_MESSAGE.lock().unwrap() = "Failed to initialize audio output.".to_string();
        }

        *AUDIO_ACTIVE.lock().unwrap() = false;
    });
}

pub fn get_audio_status() -> String {
    AUDIO_MESSAGE.lock().unwrap().clone()
}