use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Text},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

use std::{
    process::Command,
    sync::Mutex,
    thread,
    time::{Duration, Instant},
};

use once_cell::sync::Lazy;

pub static DRIVER_SELECTION_INDEX: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));
pub static DRIVER_INSTALLING: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
pub static DRIVER_SELECTION_ACTIVE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
pub static INSTALL_LOG: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
pub static INSTALL_PROGRESS: Lazy<Mutex<u16>> = Lazy::new(|| Mutex::new(0));
pub static INSTALL_START: Lazy<Mutex<Option<Instant>>> = Lazy::new(|| Mutex::new(None));
pub static SHOW_REBOOT_PROMPT: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

pub fn get_driver_list() -> Vec<String> {
    vec![
        "nvidia".to_string(),
        "nvidia-dkms".to_string(),
        "nvidia-open-dkms".to_string(),
        "nvidia-lts".to_string(),
        "nvidia-470xx-dkms".to_string(),
    ]
}

pub fn get_driver_index() -> usize {
    *DRIVER_SELECTION_INDEX.lock().unwrap()
}

pub fn check_driver_selection() -> bool {
    *DRIVER_SELECTION_ACTIVE.lock().unwrap()
}

pub fn check_driver_installing() -> bool {
    *DRIVER_INSTALLING.lock().unwrap()
}

pub fn reset_driver_state() {
    *DRIVER_SELECTION_ACTIVE.lock().unwrap() = false;
    *DRIVER_INSTALLING.lock().unwrap() = false;
    *INSTALL_PROGRESS.lock().unwrap() = 0;
    *SHOW_REBOOT_PROMPT.lock().unwrap() = false;
    *INSTALL_LOG.lock().unwrap() = String::new();
    *INSTALL_START.lock().unwrap() = None;
}

pub fn increment_driver_selection() {
    let mut index = DRIVER_SELECTION_INDEX.lock().unwrap();
    let list = get_driver_list();
    if *index < list.len().saturating_sub(1) {
        *index += 1;
    }
}

pub fn decrement_driver_selection() {
    let mut index = DRIVER_SELECTION_INDEX.lock().unwrap();
    if *index > 0 {
        *index -= 1;
    }
}

pub fn install_selected_driver() {
    let index = get_driver_index();
    let selected = get_driver_list()[index].clone();

    *DRIVER_INSTALLING.lock().unwrap() = true;
    *DRIVER_SELECTION_ACTIVE.lock().unwrap() = false;
    *INSTALL_PROGRESS.lock().unwrap() = 0;
    *INSTALL_LOG.lock().unwrap() = format!("Installing NVIDIA driver: {}\n", selected);
    *INSTALL_START.lock().unwrap() = Some(Instant::now());

    thread::spawn(move || {
        let steps = vec![
            format!("sudo aura -A {}", selected),
            "sudo mkinitcpio -P".to_string(),
            "sudo grub-mkconfig -o /boot/grub/grub.cfg".to_string(),
        ];

        for (i, cmd) in steps.iter().enumerate() {
            let result = Command::new("bash")
                .arg("-c")
                .arg(cmd)
                .output();

            let mut log = INSTALL_LOG.lock().unwrap();
            match result {
                Ok(out) => {
                    log.push_str(&format!(
                        "[✓] {}\n{}",
                        cmd,
                        String::from_utf8_lossy(&out.stdout)
                    ));
                }
                Err(e) => {
                    log.push_str(&format!("[✗] {}\nError: {}\n", cmd, e));
                    break;
                }
            }

            *INSTALL_PROGRESS.lock().unwrap() = ((i + 1) as u16 * 100 / steps.len() as u16).min(100);
        }

        *SHOW_REBOOT_PROMPT.lock().unwrap() = true;
    });
}

pub fn draw_driver_install_output(f: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.area());

    let progress = *INSTALL_PROGRESS.lock().unwrap();
    let start = *INSTALL_START.lock().unwrap();
    let elapsed = start.map(|s| s.elapsed().as_secs()).unwrap_or(0);
    let eta = if progress > 0 {
        (elapsed * 100 / progress as u64).saturating_sub(elapsed)
    } else {
        0
    };

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Installing Driver..."))
        .gauge_style(Style::default().fg(Color::Green).bg(Color::Black))
        .percent(progress);

    let time_text = format!("Elapsed: {}s | ETA: {}s", elapsed, eta);
    let time_box = Paragraph::new(Span::raw(time_text))
        .block(Block::default().title("Time"));

    let log = INSTALL_LOG.lock().unwrap();
    let log_box = Paragraph::new(Text::from(log.as_str()))
        .block(Block::default().title("Install Log").borders(Borders::ALL))
        .wrap(ratatui::widgets::Wrap { trim: false });

    let reboot_msg = if *SHOW_REBOOT_PROMPT.lock().unwrap() {
        Paragraph::new(Span::styled(
            "Install complete. Please reboot your system now.",
            Style::default().fg(Color::Yellow).bg(Color::Black),
        ))
        .block(Block::default().title("Reboot Required").borders(Borders::ALL))
    } else {
        Paragraph::new("")
    };

    f.render_widget(gauge, layout[0]);
    f.render_widget(time_box, layout[1]);
    f.render_widget(log_box, layout[2]);
    f.render_widget(reboot_msg, layout[3]);
}