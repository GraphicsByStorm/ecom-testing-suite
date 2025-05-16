use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Modifier, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Frame,
};
use once_cell::sync::Lazy;
use std::{process::Command, sync::Mutex};

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
    let mut capacity = String::from("Unknown");
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
                        capacity = format_capacity(bytes);
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
        _ => Color::Gray,
    };

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(5), Constraint::Length(7), Constraint::Min(5)])
        .split(area);

    // HEALTH INDICATOR BLOCK (top)
    let health_block = Paragraph::new(Text::from(Line::from(Span::styled(
        format!("🩺 {health}"),
        Style::default()
            .fg(Color::White)
            .bg(health_color)
            .add_modifier(Modifier::BOLD),
    ))))
    .block(Block::default().borders(Borders::ALL).title("Health Status"))
    .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(health_block, main_chunks[0]);

    // GRID OF KEY ATTRIBUTES
    let attribute_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33); 3])
        .split(main_chunks[1]);

    let info_lines = vec![
        ("Model Family", family, "🏠"),
        ("Device Model", model, "💾"),
        ("Capacity", &capacity, "💽"),
        ("Temperature (°C)", temp, "🌡"),
        ("Runtime Hours", hours, "⏱"),
    ];

    for (i, (label, value, icon)) in info_lines.iter().enumerate() {
        let paragraph = Paragraph::new(Text::from(vec![
            Line::from(Span::styled(
                icon.to_string(),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                format!("{label}: {value}"),
                Style::default().fg(Color::White),
            )),
        ]))
        .block(Block::default().borders(Borders::ALL).title(*label))
        .alignment(ratatui::layout::Alignment::Center);

        let col = i % 3;
        let row = i / 3;

        if row == 0 {
            f.render_widget(paragraph, attribute_chunks[col]);
        }
    }

    // SCROLLABLE SMART ATTRIBUTE LIST (bottom)
    let mut lines = Vec::new();
    for line in output.lines() {
        if let Some((key, value)) = line.split_once(':') {
            lines.push(Line::from(vec![
                Span::styled(format!("{}: ", key.trim()), Style::default().fg(Color::Cyan).bold()),
                Span::styled(value.trim(), Style::default().fg(Color::White)),
            ]));
        } else {
            lines.push(Line::from(Span::raw(format!(" {}", line))));
        }
    }

    let content_height = lines.len();
    let visible_height = main_chunks[2].height.saturating_sub(2);
    let max_scroll = content_height.saturating_sub(visible_height as usize);
    let scroll_pos = scroll.min(max_scroll as u16) as usize;

    let visible_lines = lines
        .iter()
        .skip(scroll_pos)
        .take(visible_height as usize)
        .cloned()
        .collect::<Vec<_>>();

    let mut scroll_state = ScrollbarState::new(content_height).position(scroll_pos);

    let smart_paragraph = Paragraph::new(Text::from(visible_lines))
        .block(Block::default().borders(Borders::ALL).title("SMART Attributes"))
        .wrap(ratatui::widgets::Wrap { trim: true });

    let scrollbar = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .track_symbol(Some("│"))
        .thumb_symbol("█");

    f.render_widget(smart_paragraph, main_chunks[2]);
    f.render_stateful_widget(scrollbar, main_chunks[2], &mut scroll_state);
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

fn detect_device_type(device: &str) -> String {
    let output = Command::new("smartctl")
        .arg("-i")
        .arg("--json=c")
        .arg(device)
        .output();

    if let Ok(out) = output {
        if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&out.stdout) {
            if let Some(device_type) = json.get("device").and_then(|d| d.get("type")).and_then(|t| t.as_str()) {
                return device_type.to_string();
            }
        }
    }

    // Fallback to "auto" if detection fails
    "auto".to_string()
}

pub fn run_smart_test_on_selected_drive() {
    let disks = DISK_LIST.lock().unwrap();
    let index = *SELECTED_DISK_INDEX.lock().unwrap();

    let fallback = String::from("/dev/sda");
    let disk_line = disks.get(index).unwrap_or(&fallback);
    let device = disk_line.split(" - ").next().unwrap_or("/dev/sda");

    let dev_type = detect_device_type(device);

    let output = Command::new("smartctl")
        .arg("-a")
        .arg("-d")
        .arg(&dev_type)
        .arg(device)
        .output();

    match output {
        Ok(out) => {
            *SMART_OUTPUT.lock().unwrap() = String::from_utf8_lossy(&out.stdout).to_string();
            *SMART_SCROLL.lock().unwrap() = 0;
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

pub fn exit_smart_output() {
    *SMART_ACTIVE.lock().unwrap() = false;
}

fn format_capacity(bytes: u64) -> String {
    const GB: u64 = 1 << 30;
    const TB: u64 = 1 << 40;
    const MB: u64 = 1 << 20;
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