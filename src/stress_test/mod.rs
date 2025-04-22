use std::sync::Mutex;
use once_cell::sync::Lazy;
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::{Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

pub static STRESS_TEST_ACTIVE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
pub static STRESS_TEST_PROGRESS: Lazy<Mutex<u16>> = Lazy::new(|| Mutex::new(0));
pub static STRESS_TEST_MESSAGE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

pub fn start_stress_test() {
    *STRESS_TEST_ACTIVE.lock().unwrap() = true;
    *STRESS_TEST_PROGRESS.lock().unwrap() = 0;
    *STRESS_TEST_MESSAGE.lock().unwrap() = "Running stress test...".to_string();

    std::thread::spawn(|| {
        for i in 0..=100 {
            *STRESS_TEST_PROGRESS.lock().unwrap() = i;
            std::thread::sleep(std::time::Duration::from_millis(50));
        }

        *STRESS_TEST_MESSAGE.lock().unwrap() = "Stress test completed.".to_string();
    });
}

pub fn check_stress_active() -> bool {
    *STRESS_TEST_ACTIVE.lock().unwrap()
}

pub fn stop_stress_test() {
    *STRESS_TEST_ACTIVE.lock().unwrap() = false;
    *STRESS_TEST_PROGRESS.lock().unwrap() = 0;
    *STRESS_TEST_MESSAGE.lock().unwrap() = String::new();
}

pub fn draw_stress_test_popup(f: &mut Frame) {
    let progress = *STRESS_TEST_PROGRESS.lock().unwrap();
    let message = STRESS_TEST_MESSAGE.lock().unwrap();
    let area = f.area();

    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .margin(2)
        .split(area);

    let gauge = Gauge::default()
        .block(Block::default().title("Stress Test Progress").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Green).bg(Color::Black))
        .percent(progress);

    let paragraph = Paragraph::new(Span::raw(message.as_str()))
        .block(Block::default().title("Status").borders(Borders::ALL));

    f.render_widget(gauge, chunks[0]);
    f.render_widget(paragraph, chunks[1]);
}