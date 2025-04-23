use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Gauge, List, ListItem, ListState, Paragraph},
    Frame,
};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

use once_cell::sync::Lazy;

static DEVICES: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(vec![]));
static DEVICE_INDEX: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));
static MESSAGE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
pub static AUDIO_TEST_ACTIVE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
pub static AUDIO_TEST_PROGRESS: Lazy<Mutex<u16>> = Lazy::new(|| Mutex::new(0));
pub static AUDIO_TEST_MESSAGE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
pub static AUDIO_DEVICES: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(vec![]));
pub static AUDIO_DEVICE_INDEX: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

pub fn check_audio_test_active() -> bool {
    *AUDIO_TEST_ACTIVE.lock().unwrap()
}

pub fn enter_audio_test() {
    *AUDIO_DEVICES.lock().unwrap() = vec![
        "Speakers".to_string(),
        "Headphones".to_string(),
    ];
    *AUDIO_DEVICE_INDEX.lock().unwrap() = 0;
    *AUDIO_TEST_ACTIVE.lock().unwrap() = true;
    *AUDIO_TEST_PROGRESS.lock().unwrap() = 0;
    *AUDIO_TEST_MESSAGE.lock().unwrap() = "Playing audio test...".to_string();

    thread::spawn(|| {
        if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
            if let Ok(file) = File::open("assets/audio/test.wav") {
                if let Ok(source) = Decoder::new(BufReader::new(file)) {
                    let sink = Sink::try_new(&stream_handle).unwrap();
                    sink.append(source);
                    sink.sleep_until_end();
                }
            }
        }

        for i in 0..=100 {
            *AUDIO_TEST_PROGRESS.lock().unwrap() = i;
            thread::sleep(Duration::from_millis(20));
        }

        *AUDIO_TEST_MESSAGE.lock().unwrap() = "Audio test completed.".to_string();
    });
}

pub fn exit_audio_test() {
    *AUDIO_TEST_ACTIVE.lock().unwrap() = false;
    *AUDIO_TEST_PROGRESS.lock().unwrap() = 0;
    *AUDIO_TEST_MESSAGE.lock().unwrap() = String::new();
}

pub fn increment_device_selection() {
    let mut index = AUDIO_DEVICE_INDEX.lock().unwrap();
    let devices = AUDIO_DEVICES.lock().unwrap();
    if *index < devices.len().saturating_sub(1) {
        *index += 1;
    }
}

pub fn decrement_device_selection() {
    let mut index = AUDIO_DEVICE_INDEX.lock().unwrap();
    if *index > 0 {
        *index -= 1;
    }
}

pub fn run_audio_test() {
    let devices = DEVICES.lock().unwrap();
    let index = *DEVICE_INDEX.lock().unwrap();

    let fallback = "No device selected.".to_string();
    let selected = devices.get(index).unwrap_or(&fallback);

    let mut msg = MESSAGE.lock().unwrap();
    *msg = format!("Starting test on: {}\n(Audio response test in development)", selected);
}

pub fn draw_audio_test(f: &mut Frame) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Length(3), Constraint::Min(1)])
        .split(area);

    let devices = AUDIO_DEVICES.lock().unwrap();
    let selected = *AUDIO_DEVICE_INDEX.lock().unwrap();
    let progress = *AUDIO_TEST_PROGRESS.lock().unwrap();
    let message = AUDIO_TEST_MESSAGE.lock().unwrap();

    let mut state = ListState::default();
    state.select(Some(selected));

    let items: Vec<ListItem> = devices.iter().map(|d| ListItem::new(Span::raw(d.clone()))).collect();
    let list = List::new(items)
        .block(Block::default().title("Select Audio Output").borders(Borders::ALL))
        .highlight_style(Style::default().bg(Color::White).fg(Color::Black))
        .highlight_symbol("▶ ");

    let gauge = Gauge::default()
        .block(Block::default().title("Progress").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Green).bg(Color::Black))
        .percent(progress);

    let paragraph = Paragraph::new(Span::raw(message.as_str()))
        .block(Block::default().borders(Borders::ALL).title("Audio Test Info"));

    f.render_stateful_widget(list, chunks[0], &mut state);
    f.render_widget(gauge, chunks[1]);
    f.render_widget(paragraph, chunks[2]);
}