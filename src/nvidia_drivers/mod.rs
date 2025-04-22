use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

use once_cell::sync::Lazy;
use std::{
    process::Command,
    sync::Mutex,
    thread,
    time::{Duration, Instant},
};

static DRIVER_SELECTION_ACTIVE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
static SELECTED_DRIVER_INDEX: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));
static INSTALL_IN_PROGRESS: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
static INSTALL_PROGRESS: Lazy<Mutex<u16>> = Lazy::new(|| Mutex::new(0));
static INSTALL_MESSAGE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

pub fn get_driver_list() -> Vec<String> {
    vec![
        "nvidia (stable)".to_string(),
        "nvidia-beta".to_string(),
        "nvidia-open".to_string(),
        "nvidia-390xx".to_string(),
    ]
}

pub fn get_driver_index() -> usize {
    *SELECTED_DRIVER_INDEX.lock().unwrap()
}

pub fn check_driver_selection() -> bool {
    *DRIVER_SELECTION_ACTIVE.lock().unwrap()
}

pub fn check_driver_installing() -> bool {
    *INSTALL_IN_PROGRESS.lock().unwrap()
}

pub fn enter_driver_selection() {
    *DRIVER_SELECTION_ACTIVE.lock().unwrap() = true;
    *SELECTED_DRIVER_INDEX.lock().unwrap() = 0;
    *INSTALL_MESSAGE.lock().unwrap() = "Select a driver to install".to_string();
}

pub fn exit_driver_selection() {
    *DRIVER_SELECTION_ACTIVE.lock().unwrap() = false;
    *INSTALL_IN_PROGRESS.lock().unwrap() = false;
    *INSTALL_PROGRESS.lock().unwrap() = 0;
    *INSTALL_MESSAGE.lock().unwrap() = String::new();
}

pub fn increment_driver_selection() {
    let mut index = SELECTED_DRIVER_INDEX.lock().unwrap();
    let drivers = get_driver_list();
    if *index < drivers.len().saturating_sub(1) {
        *index += 1;
    }
}

pub fn decrement_driver_selection() {
    let mut index = SELECTED_DRIVER_INDEX.lock().unwrap();
    if *index > 0 {
        *index -= 1;
    }
}

pub fn install_selected_driver() {
    *INSTALL_IN_PROGRESS.lock().unwrap() = true;
    *INSTALL_PROGRESS.lock().unwrap() = 0;
    *INSTALL_MESSAGE.lock().unwrap() = "Installing driver...".to_string();

    let driver_name = get_driver_list()[*SELECTED_DRIVER_INDEX.lock().unwrap()].clone();

    thread::spawn(move || {
        let start = Instant::now();
        *INSTALL_MESSAGE.lock().unwrap() = format!("Installing: {}", driver_name);

        let output = Command::new("bash")
            .arg("-c")
            .arg(format!(
                r#"
set -e
sudo aura -A --noconfirm {}
sudo mkinitcpio -P
"#,
                driver_name
            ))
            .output();

        for i in 0..=100 {
            *INSTALL_PROGRESS.lock().unwrap() = i;
            thread::sleep(Duration::from_millis(40));
        }

        match output {
            Ok(out) => {
                let summary = format!(
                    "Driver installed successfully in {:.1}s\n\n{}\n\nReboot required.",
                    start.elapsed().as_secs_f32(),
                    String::from_utf8_lossy(&out.stdout)
                );
                *INSTALL_MESSAGE.lock().unwrap() = summary;
            }
            Err(e) => {
                *INSTALL_MESSAGE.lock().unwrap() = format!("Driver install failed: {}", e);
            }
        }

        *INSTALL_IN_PROGRESS.lock().unwrap() = true;
    });
}

pub fn draw_driver_install_output(f: &mut Frame) {
    let area = f.area();
    let progress = *INSTALL_PROGRESS.lock().unwrap();
    let message = INSTALL_MESSAGE.lock().unwrap();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(area);

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Install Progress"))
        .gauge_style(Style::default().fg(Color::Green).bg(Color::Black))
        .percent(progress);

    let paragraph = Paragraph::new(Span::raw(message.clone()))
        .block(Block::default().title("Status").borders(Borders::ALL));

    f.render_widget(gauge, layout[0]);
    f.render_widget(paragraph, layout[1]);
}

pub fn reset_driver_state() {
    exit_driver_selection();
}