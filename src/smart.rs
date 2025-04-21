use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, ListState},
    Frame,
};
use std::{fs, process::Command, sync::Mutex};
use once_cell::sync::Lazy;
use crate::theme::*;

pub static SMART_OUTPUT: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
pub static SMART_ACTIVE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
pub static DISK_SELECT: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
pub static DISK_INDEX: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));
pub static DISKS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static SCROLL_POS: Lazy<Mutex<u16>> = Lazy::new(|| Mutex::new(0));

#[derive(Debug, Default, Clone)]
pub struct SmartSummary {
    pub model_family: String,
    pub device_model: String,
    pub capacity: String,
    pub power_on_hours: String,
    pub drive_health: String,
    pub health_color: Color,
    pub temperature: String,
}

pub static SUMMARY_INFO: Lazy<Mutex<SmartSummary>> = Lazy::new(|| Mutex::new(SmartSummary::default()));

pub fn check_smart_active() -> bool {
    *SMART_ACTIVE.lock().unwrap()
}

pub fn check_smart_disk_select() -> bool {
    *DISK_SELECT.lock().unwrap()
}

pub fn exit_smart_output() {
    *SMART_ACTIVE.lock().unwrap() = false;
    *DISK_SELECT.lock().unwrap() = true;
    *SCROLL_POS.lock().unwrap() = 0;
}

pub fn exit_disk_selection() {
    *DISK_SELECT.lock().unwrap() = false;
}

pub fn enter_disk_selection() {
    *DISKS.lock().unwrap() = get_disks();
    *DISK_INDEX.lock().unwrap() = 0;
    *DISK_SELECT.lock().unwrap() = true;
    *SMART_ACTIVE.lock().unwrap() = false;
}

pub fn increment_disk_selection() {
    let mut index = DISK_INDEX.lock().unwrap();
    let disks = DISKS.lock().unwrap();
    if *index < disks.len().saturating_sub(1) {
        *index += 1;
    }
}

pub fn decrement_disk_selection() {
    let mut index = DISK_INDEX.lock().unwrap();
    if *index > 0 {
        *index -= 1;
    }
}

pub fn scroll_output_down() {
    let mut scroll = SCROLL_POS.lock().unwrap();
    *scroll = scroll.saturating_add(1);
}

pub fn scroll_output_up() {
    let mut scroll = SCROLL_POS.lock().unwrap();
    *scroll = scroll.saturating_sub(1);
}

pub fn run_selected_smart() {
    let index = *DISK_INDEX.lock().unwrap();
    let disks = DISKS.lock().unwrap();
    let device = &disks[index];

    match Command::new("smartctl").args(["-a", device]).output() {
        Ok(output) => {
            let data = String::from_utf8_lossy(&output.stdout).to_string();
            *SMART_OUTPUT.lock().unwrap() = data.clone();
            *SUMMARY_INFO.lock().unwrap() = extract_summary_info(&data);
            *SMART_ACTIVE.lock().unwrap() = true;
            *DISK_SELECT.lock().unwrap() = false;
        }
        Err(e) => {
            *SMART_OUTPUT.lock().unwrap() = format!("Failed to run smartctl on {}: {}", device, e);
            *SMART_ACTIVE.lock().unwrap() = true;
            *DISK_SELECT.lock().unwrap() = false;
        }
    }
}

pub fn draw_smart_output(f: &mut Frame) {
    let size = f.area();
    let scroll = *SCROLL_POS.lock().unwrap();
    let output = SMART_OUTPUT.lock().unwrap();
    let summary = SUMMARY_INFO.lock().unwrap();

    let layout_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(9),
            Constraint::Min(10),
        ])
        .split(size);

    let top = layout_chunks[0];
    let middle = layout_chunks[1];
    let bottom = layout_chunks[2];

    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(middle);

    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(middle_chunks[0]);

    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(middle_chunks[1]);

    let health_box = Paragraph::new(Line::from(vec![
        Span::styled(
            format!(" DRIVE HEALTH: {} ", summary.drive_health),
            Style::default().fg(Color::Black).bg(summary.health_color),
        ),
    ]))
    .block(Block::default().borders(Borders::ALL).title("STATUS"));

    let model_family = Paragraph::new(Line::from(vec![
        Span::styled("\u{f02b} ", Style::default().fg(Color::Cyan)),
        Span::raw(&summary.model_family),
    ])).block(Block::default().borders(Borders::ALL).title("FAMILY"));

    let device_model = Paragraph::new(Line::from(vec![
        Span::styled("\u{f109} ", Style::default().fg(Color::Cyan)),
        Span::raw(&summary.device_model),
    ])).block(Block::default().borders(Borders::ALL).title("MODEL"));

    let capacity = Paragraph::new(Line::from(vec![
        Span::styled("\u{f1c0} ", Style::default().fg(Color::Cyan)),
        Span::raw(&summary.capacity),
    ])).block(Block::default().borders(Borders::ALL).title("CAPACITY"));

    let hours = Paragraph::new(Line::from(vec![
        Span::styled("\u{f017} ", Style::default().fg(Color::Cyan)),
        Span::raw(format!("{} hrs", summary.power_on_hours)),
    ])).block(Block::default().borders(Borders::ALL).title("RUNTIME"));

    let temperature = Paragraph::new(Line::from(vec![
        Span::styled("\u{f2c9} ", Style::default().fg(Color::Cyan)),
        Span::raw(&summary.temperature),
    ])).block(Block::default().borders(Borders::ALL).title("TEMP"));

    let output_paragraph = Paragraph::new(output.as_str())
        .block(Block::default().borders(Borders::ALL).title("FULL SMART OUTPUT (q to go back, j/k scroll)"))
        .scroll((scroll, 0))
        .wrap(ratatui::widgets::Wrap { trim: false });

    f.render_widget(health_box, top);
    f.render_widget(model_family, left[0]);
    f.render_widget(device_model, left[1]);
    f.render_widget(capacity, left[2]);
    f.render_widget(hours, right[0]);
    f.render_widget(temperature, right[1]);
    f.render_widget(output_paragraph, bottom);
}

pub fn draw_disk_selection(f: &mut Frame) {
    let size = f.area();
    let disks = DISKS.lock().unwrap();
    let index = *DISK_INDEX.lock().unwrap();

    let items: Vec<ListItem> = disks
        .iter()
        .map(|d| ListItem::new(Line::from(Span::raw(d.clone()))))
        .collect();

    let mut state = ListState::default();
    state.select(Some(index));

    let list = List::new(items)
        .block(Block::default().title(" SELECT A DRIVE ").borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, size, &mut state);
}

fn get_disks() -> Vec<String> {
    let mut disks = Vec::new();
    if let Ok(entries) = fs::read_dir("/dev") {
        for entry in entries.flatten() {
            if let Ok(fname) = entry.file_name().into_string() {
                if fname.starts_with("sd") && fname.len() == 3 {
                    disks.push(format!("/dev/{}", fname));
                }
            }
        }
    }
    disks
}

fn extract_summary_info(output: &str) -> SmartSummary {
    let mut info = SmartSummary::default();

    for line in output.lines() {
        if line.contains("Model Family") {
            info.model_family = line.split(':').nth(1).unwrap_or("").trim().to_string();
        } else if line.contains("Device Model") {
            info.device_model = line.split(':').nth(1).unwrap_or("").trim().to_string();
        } else if line.contains("User Capacity") {
            info.capacity = line.split(':').nth(1).unwrap_or("").trim().to_string();
        } else if line.contains("Power_On_Hours") {
            info.power_on_hours = line.split_whitespace().last().unwrap_or("?").to_string();
        } else if line.contains("Temperature_Celsius") {
            info.temperature = line.split_whitespace().last().unwrap_or("?").to_string();
        } else if line.contains("SMART overall-health self-assessment test result") {
            if line.contains("PASSED") {
                info.drive_health = "GREAT".to_string();
                info.health_color = Color::Green;
            } else if line.contains("OK") {
                info.drive_health = "GOOD".to_string();
                info.health_color = Color::Yellow;
            } else {
                info.drive_health = "BAD".to_string();
                info.health_color = Color::Red;
            }
        }
    }

    info
}