use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use once_cell::sync::Lazy;
use std::process::Command;
use std::sync::Mutex;

pub static SMART_OUTPUT: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
pub static SMART_ACTIVE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
pub static DISK_SELECTION_ACTIVE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
pub static DISK_LIST: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(vec![]));
pub static SELECTED_DISK_INDEX: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

pub fn check_disk_selection_active() -> bool {
    *DISK_SELECTION_ACTIVE.lock().unwrap()
}

pub fn check_smart_active() -> bool {
    *SMART_ACTIVE.lock().unwrap()
}

pub fn enter_disk_selection() {
    let output = Command::new("lsblk")
        .arg("-d")
        .arg("-e7") // exclude loop devices
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
                Some(format!("/dev/{} - {} - {}", parts[0], parts[1], parts[2..].join(" ")))
            } else {
                None
            }
        })
        .collect();

    *DISK_LIST.lock().unwrap() = list;
    *SELECTED_DISK_INDEX.lock().unwrap() = 0;
    *DISK_SELECTION_ACTIVE.lock().unwrap() = true;
}

pub fn exit_disk_selection() {
    *DISK_SELECTION_ACTIVE.lock().unwrap() = false;
}

pub fn draw_disk_selection(f: &mut Frame) {
    let area = f.area();
    let disks = DISK_LIST.lock().unwrap();
    let selected = *SELECTED_DISK_INDEX.lock().unwrap();

    let items: Vec<ListItem> = disks
        .iter()
        .map(|d| ListItem::new(Span::raw(d.clone())))
        .collect();

    let mut state = ListState::default();
    state.select(Some(selected));

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(area);

    let list = List::new(items)
        .block(Block::default().title("Select Disk").borders(Borders::ALL))
        .highlight_style(Style::default().bg(Color::White).fg(Color::Black))
        .highlight_symbol("▶ ");

    let info = Paragraph::new(Span::raw(
        "Use ↑/↓ to choose a disk. Press Enter to run SMART test. Press q to cancel.",
    ))
    .block(Block::default().borders(Borders::ALL).title("Instructions"));

    f.render_stateful_widget(list, layout[0], &mut state);
    f.render_widget(info, layout[1]);
}

pub fn draw_smart_output(f: &mut Frame) {
    let area = f.area();
    let output = SMART_OUTPUT.lock().unwrap();

    let block = Block::default().title("SMART Test Result").borders(Borders::ALL);
    let paragraph = Paragraph::new(Span::raw(output.as_str())).block(block);
    f.render_widget(paragraph, area);
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
        }
        Err(e) => {
            *SMART_OUTPUT.lock().unwrap() = format!("Failed to run smartctl: {}", e);
        }
    }

    *SMART_ACTIVE.lock().unwrap() = true;
    *DISK_SELECTION_ACTIVE.lock().unwrap() = false;
}

pub fn exit_smart_output() {
    *SMART_ACTIVE.lock().unwrap() = false;
}