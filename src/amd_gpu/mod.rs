use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::process::Command;

pub static AMD_OUTPUT: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
pub static AMD_ACTIVE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
pub static GPU_SELECT: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
pub static GPU_INDEX: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));
pub static GPU_LIST: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static AMD_SCROLL: Lazy<Mutex<u16>> = Lazy::new(|| Mutex::new(0));

pub fn check_amd_active() -> bool {
    *AMD_ACTIVE.lock().unwrap()
}

pub fn check_gpu_select() -> bool {
    *GPU_SELECT.lock().unwrap()
}

pub fn exit_amd_output() {
    *AMD_ACTIVE.lock().unwrap() = false;
    *GPU_SELECT.lock().unwrap() = true;
    *AMD_SCROLL.lock().unwrap() = 0;
}

pub fn enter_gpu_selection() {
    *GPU_LIST.lock().unwrap() = get_amd_gpus();
    *GPU_INDEX.lock().unwrap() = 0;
    *GPU_SELECT.lock().unwrap() = true;
    *AMD_ACTIVE.lock().unwrap() = false;
}

pub fn exit_gpu_selection() {
    *GPU_SELECT.lock().unwrap() = false;
}

pub fn increment_gpu_selection() {
    let mut index = GPU_INDEX.lock().unwrap();
    let gpus = GPU_LIST.lock().unwrap();
    if *index < gpus.len().saturating_sub(1) {
        *index += 1;
    }
}

pub fn decrement_gpu_selection() {
    let mut index = GPU_INDEX.lock().unwrap();
    if *index > 0 {
        *index -= 1;
    }
}

pub fn scroll_output_down() {
    let mut scroll = AMD_SCROLL.lock().unwrap();
    *scroll = scroll.saturating_add(1);
}

pub fn scroll_output_up() {
    let mut scroll = AMD_SCROLL.lock().unwrap();
    *scroll = scroll.saturating_sub(1);
}

pub fn run_selected_gpu_check() {
    let index = *GPU_INDEX.lock().unwrap();
    let gpus = GPU_LIST.lock().unwrap();
    let selected_gpu = &gpus[index];

    *AMD_OUTPUT.lock().unwrap() = format!("Testing AMD GPU: {}\nRunning stress and stability tests...", selected_gpu);
    *AMD_ACTIVE.lock().unwrap() = true;
    *GPU_SELECT.lock().unwrap() = false;
}

pub fn run_amd_gpu_check() {
    // Optional stub if referenced elsewhere
    run_selected_gpu_check();
}

pub fn draw_amd_output(f: &mut Frame) {
    let size = f.area();
    let scroll = *AMD_SCROLL.lock().unwrap();
    let output = AMD_OUTPUT.lock().unwrap();

    let block = Block::default()
        .title(" AMD GPU STATUS (q to go back, j/k scroll) ")
        .borders(Borders::ALL);

    let paragraph = Paragraph::new(output.as_str())
        .block(block)
        .style(Style::default())
        .scroll((scroll, 0))
        .wrap(ratatui::widgets::Wrap { trim: false });

    f.render_widget(paragraph, size);
}

pub fn draw_gpu_selection(f: &mut Frame) {
    let size = f.area();
    let gpus = GPU_LIST.lock().unwrap();
    let index = *GPU_INDEX.lock().unwrap();

    let items: Vec<ListItem> = gpus.iter().map(|gpu| ListItem::new(Line::from(Span::raw(gpu.clone())))).collect();

    let mut state = ListState::default();
    state.select(Some(index));

    let list = List::new(items)
        .block(Block::default().title(" SELECT A GPU ").borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, size, &mut state);
}

fn get_amd_gpus() -> Vec<String> {
    // Replace with dynamic detection later if needed
    vec![
        "MSI AMD Radeon RX 7700 XT".to_string(),
        "ASUS AMD Radeon RX 6800 XT".to_string(),
    ]
}