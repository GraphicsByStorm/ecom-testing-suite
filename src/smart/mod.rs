use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};

use ratatui::prelude::Stylize;
use once_cell::sync::Lazy;
use std::process::Command;
use std::sync::Mutex;

pub static SMART_OUTPUT: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
pub static SMART_ACTIVE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
pub static DISK_SELECTION_ACTIVE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
pub static DISK_LIST: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(vec![]));
pub static SELECTED_DISK_INDEX: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));
pub static SMART_SCROLL: Lazy<Mutex<u16>> = Lazy::new(|| Mutex::new(0));

pub fn check_disk_selection_active() -> bool {
    *DISK_SELECTION_ACTIVE.lock().unwrap()
}

pub fn enter_disk_selection() {
    let output = Command::new("lsblk")
        .arg("-d")
        .arg("-o")
        .arg("NAME,SIZE,MODEL")
        .output()
        .expect("Failed to run lsblk");

    let list = String::from_utf8_lossy(&output.stdout)
        .lines()
        .skip(1)
        .filter_map(|line| {
            let parts: Vec<_> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                Some(format!("/dev/{} - {} - {}", parts[0], parts[1], parts[2]))
            } else {
                None
            }
        })
        .collect();

    *DISK_LIST.lock().unwrap() = list;
    *SELECTED_DISK_INDEX.lock().unwrap() = 0;
    *DISK_SELECTION_ACTIVE.lock().unwrap() = true;
}

pub fn get_drive_list() -> Vec<String> {
    DISK_LIST.lock().unwrap().clone()
}

pub fn get_selected_drive_index() -> usize {
    *SELECTED_DISK_INDEX.lock().unwrap()
}

pub fn exit_disk_selection() {
    *DISK_SELECTION_ACTIVE.lock().unwrap() = false;
}

pub fn draw_smart_output(f: &mut Frame) {
    let area = f.area();
    let output = SMART_OUTPUT.lock().unwrap().clone();
    let scroll = *SMART_SCROLL.lock().unwrap();

    let mut health = "Unknown";
    let mut family = "Unknown";
    let mut model = "Unknown";
    let mut capacity = "Unknown";
    let mut temp = "N/A";
    let mut hours = "Unknown";

    for line in output.lines() {
        if line.contains("Device Model:") {
            model = line.split(':').nth(1).unwrap_or("").trim();
        }
        if line.contains("Model Family:") {
            family = line.split(':').nth(1).unwrap_or("").trim();
        }
        if line.contains("User Capacity:") {
            if let Some(start) = line.find('[') {
                if let Some(end) = line.find(']') {
                    let raw_str = &line[start + 1..end];
                    let sanitized = raw_str.replace(",", "").replace(" bytes", "").trim().to_string();
                    if let Ok(bytes) = sanitized.parse::<u64>() {
                        let formatted = format_capacity(bytes);
                        capacity = Box::leak(formatted.into_boxed_str());
                    }
                }
            }
        }
        if line.contains("Temperature_Celsius") {
            temp = line.split_whitespace().last().unwrap_or("N/A");
        }
        if line.contains("Power_On_Hours") {
            hours = line.split_whitespace().last().unwrap_or("Unknown");
        }
        if line.contains("SMART overall-health self-assessment test result:") {
            let result = line.split(':').nth(1).unwrap_or("").trim();
            health = match result {
                "PASSED" => "Great",
                "OK" => "Good",
                _ => "Bad",
            };
        }
    }

    let health_color = match health {
        "Great" => Color::Green,
        "Good" => Color::Yellow,
        "Bad" => Color::Red,
        _ => Color::White,
    };

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    let grid_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[0]);

    let top_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(34), Constraint::Percentage(33)])
        .split(grid_chunks[0]);

    let bottom_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(34), Constraint::Percentage(33)])
        .split(grid_chunks[1]);

    let boxes = [
        ("Health Status", health, health_color, "🩺", top_row[0]),
        ("Model Family", family, Color::White, "🏠", top_row[1]),
        ("Device Model", model, Color::White, "💾", top_row[2]),
        ("Capacity", capacity, Color::White, "💽", bottom_row[0]),
        ("Temperature (°C)", temp, Color::White, "🌡", bottom_row[1]),
        ("Runtime Hours", hours, Color::White, "⏱", bottom_row[2]),
    ];

    for (label, value, color, icon, rect) in boxes.iter() {
        let text = Text::from(Span::styled(format!("{} {}", icon, value), Style::default().fg(*color).bold()));
        let block = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title(*label));
        f.render_widget(block, *rect);
    }

    let mut lines = Vec::new();
    for line in output.lines() {
        if let Some((key, value)) = line.split_once(':') {
            lines.push(Line::from(Span::raw(format!("{}: {}", key.trim(), value.trim()))));
        } else {
            lines.push(Line::from(Span::raw(line)));
        }
    }

    let paragraph = Paragraph::new(Text::from(lines.clone()))
        .block(Block::default().title("Full SMART Report").borders(Borders::ALL))
        .scroll((scroll, 0));

    let mut scroll_state = ScrollbarState::new(lines.len());
    scroll_state = scroll_state.position(scroll as usize);

    let scrollbar = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .track_symbol(Some("│"))
        .thumb_symbol("█");

    f.render_widget(paragraph, main_chunks[1]);
    f.render_stateful_widget(scrollbar, main_chunks[1], &mut scroll_state);
}

pub fn scroll_up() {
    let mut scroll = SMART_SCROLL.lock().unwrap();
    if *scroll > 0 {
        *scroll -= 1;
    }
}

pub fn scroll_down() {
    let mut scroll = SMART_SCROLL.lock().unwrap();
    *scroll += 1;
}

pub fn previous_drive() {
    let mut index = SELECTED_DISK_INDEX.lock().unwrap();
    if *index > 0 {
        *index -= 1;
    }
}

pub fn next_drive() {
    let mut index = SELECTED_DISK_INDEX.lock().unwrap();
    let disks = DISK_LIST.lock().unwrap();
    if *index < disks.len().saturating_sub(1) {
        *index += 1;
    }
}

pub fn run_smart_test_on_selected_drive() {
    let disks = DISK_LIST.lock().unwrap();
    let index = *SELECTED_DISK_INDEX.lock().unwrap();

    let fallback = String::from("/dev/sda");
    let disk_line = disks.get(index).unwrap_or(&fallback);
    let device = disk_line.split(" - ").next().unwrap_or("/dev/sda");

    let output = Command::new("smartctl")
        .arg("-a")
        .arg(device)
        .output();

    match output {
        Ok(out) => {
            *SMART_OUTPUT.lock().unwrap() = String::from_utf8_lossy(&out.stdout).to_string();
            *SMART_SCROLL.lock().unwrap() = 0; // Reset scroll
        }
        Err(e) => {
            *SMART_OUTPUT.lock().unwrap() = format!("Failed to run smartctl: {}", e);
        }
    }

    *SMART_ACTIVE.lock().unwrap() = true;
    *DISK_SELECTION_ACTIVE.lock().unwrap() = false;
}

pub fn check_smart_active() -> bool {
    *SMART_ACTIVE.lock().unwrap()
}

fn format_capacity(bytes: u64) -> String {
    const KB: u64 = 1 << 10;
    const MB: u64 = 1 << 20;
    const GB: u64 = 1 << 30;
    const TB: u64 = 1 << 40;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

pub fn exit_smart_output() {
    *SMART_ACTIVE.lock().unwrap() = false;
}