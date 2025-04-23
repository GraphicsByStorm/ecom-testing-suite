use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::process::Command;

static DEVICES: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(vec![]));
static DEVICE_INDEX: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));
static ACTIVE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
static MESSAGE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

pub fn check_gamepad_test_active() -> bool {
    *ACTIVE.lock().unwrap()
}

pub fn exit_gamepad_test() {
    *ACTIVE.lock().unwrap() = false;
    *MESSAGE.lock().unwrap() = String::new();
}

pub fn enter_gamepad_test() {
    *DEVICES.lock().unwrap() = fetch_gamepad_devices();
    *DEVICE_INDEX.lock().unwrap() = 0;
    *ACTIVE.lock().unwrap() = true;
    *MESSAGE.lock().unwrap() = "Select your controller device. Full testing coming soon.".to_string();
}

pub fn increment_device_selection() {
    let mut index = DEVICE_INDEX.lock().unwrap();
    let devices = DEVICES.lock().unwrap();
    if *index < devices.len().saturating_sub(1) {
        *index += 1;
    }
}

pub fn decrement_device_selection() {
    let mut index = DEVICE_INDEX.lock().unwrap();
    if *index > 0 {
        *index -= 1;
    }
}

pub fn run_gamepad_test() {
    let devices = DEVICES.lock().unwrap();
    let index = *DEVICE_INDEX.lock().unwrap();

    let fallback = "No device selected.".to_string();
    let selected = devices.get(index).unwrap_or(&fallback);

    let mut msg = MESSAGE.lock().unwrap();
    *msg = format!("Running test on: {}\n(Gamepad button test in development)", selected);
}

pub fn draw_gamepad_test(f: &mut Frame) {
    let size = f.area();
    let devices = DEVICES.lock().unwrap();
    let selected = *DEVICE_INDEX.lock().unwrap();
    let message = MESSAGE.lock().unwrap();

    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .margin(2)
        .split(size);

    let items: Vec<ListItem> = devices
        .iter()
        .map(|d| ListItem::new(Span::raw(d.clone())))
        .collect();

    let mut state = ListState::default();
    state.select(Some(selected));

    let list = List::new(items)
        .block(Block::default().title("Select Gamepad Device").borders(Borders::ALL))
        .highlight_style(Style::default().bg(Color::White).fg(Color::Black))
        .highlight_symbol("▶ ");

    let info = Paragraph::new(Span::raw(message.as_str()))
        .block(Block::default().title("Gamepad Test Info").borders(Borders::ALL));

    f.render_stateful_widget(list, chunks[0], &mut state);
    f.render_widget(info, chunks[1]);
}

fn fetch_gamepad_devices() -> Vec<String> {
    let output = Command::new("lsusb")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_else(|_| "Failed to run lsusb".to_string());

    output
        .lines()
        .filter(|line| {
            let lower = line.to_lowercase();
            lower.contains("controller") || lower.contains("gamepad") || lower.contains("joystick")
        })
        .map(String::from)
        .collect()
}