use std::process::Command;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::theme::*;

pub static AMD_OUTPUT: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
pub static AMD_ACTIVE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
pub static AMD_SCROLL: Lazy<Mutex<u16>> = Lazy::new(|| Mutex::new(0));

pub static AMD_GPUS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(vec!["MSI AMD Radeon RX 7700 XT".to_string()]));
pub static AMD_GPU_INDEX: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));
pub static GPU_SELECT_MODE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

pub fn check_amd_active() -> bool {
    *AMD_ACTIVE.lock().unwrap()
}

pub fn check_gpu_select() -> bool {
    *GPU_SELECT_MODE.lock().unwrap()
}

pub fn enter_gpu_selection() {
    *GPU_SELECT_MODE.lock().unwrap() = true;
}

pub fn exit_gpu_selection() {
    *GPU_SELECT_MODE.lock().unwrap() = false;
}

pub fn increment_gpu_selection() {
    let mut index = AMD_GPU_INDEX.lock().unwrap();
    let total = AMD_GPUS.lock().unwrap().len();
    *index = (*index + 1).min(total - 1);
}

pub fn decrement_gpu_selection() {
    let mut index = AMD_GPU_INDEX.lock().unwrap();
    *index = index.saturating_sub(1);
}

pub fn scroll_output_down() {
    let mut scroll = AMD_SCROLL.lock().unwrap();
    *scroll = scroll.saturating_add(1);
}

pub fn scroll_output_up() {
    let mut scroll = AMD_SCROLL.lock().unwrap();
    *scroll = scroll.saturating_sub(1);
}

pub fn exit_amd_output() {
    *AMD_ACTIVE.lock().unwrap() = false;
    *AMD_SCROLL.lock().unwrap() = 0;
}

pub fn run_selected_gpu_check() {
    let gpus = AMD_GPUS.lock().unwrap();
    let index = *AMD_GPU_INDEX.lock().unwrap();
    let selected = gpus.get(index).unwrap_or(&"Unknown GPU".to_string()).clone();

    let output = Command::new("radeontop")
        .arg("-d")
        .arg("-")
        .arg("-l")
        .arg("1")
        .output();

    match output {
        Ok(out) => {
            *AMD_OUTPUT.lock().unwrap() = format!("Testing GPU: {}\n{}", selected, String::from_utf8_lossy(&out.stdout));
            *AMD_ACTIVE.lock().unwrap() = true;
            *GPU_SELECT_MODE.lock().unwrap() = false;
        }
        Err(e) => {
            *AMD_OUTPUT.lock().unwrap() = format!("Failed to run radeontop on {}: {}", selected, e);
            *AMD_ACTIVE.lock().unwrap() = true;
            *GPU_SELECT_MODE.lock().unwrap() = false;
        }
    }
}

pub fn draw_amd_output(f: &mut Frame) {
    let area = f.area();
    let scroll = *AMD_SCROLL.lock().unwrap();
    let output = AMD_OUTPUT.lock().unwrap();

    let block = Block::default()
        .title(" AMD GPU Output (q to go back, j/k to scroll) ")
        .borders(Borders::ALL);

    let paragraph = Paragraph::new(output.as_str())
        .block(block)
        .style(normal_text_style())
        .scroll((scroll, 0))
        .wrap(ratatui::widgets::Wrap { trim: false });

    f.render_widget(paragraph, area);
}

pub fn draw_gpu_selection(f: &mut Frame) {
    let area = f.area();
    let gpus = AMD_GPUS.lock().unwrap();
    let index = *AMD_GPU_INDEX.lock().unwrap();

    let items: Vec<ListItem> = gpus
        .iter()
        .enumerate()
        .map(|(_, g)| ListItem::new(Span::raw(g.clone())))
        .collect();

    let list = List::new(items)
        .block(menu_block(" SELECT A GPU "))
        .highlight_style(highlight_style())
        .highlight_symbol(">> ");

    let mut state = ratatui::widgets::ListState::default();
    state.select(Some(index));
    f.render_stateful_widget(list, area, &mut state);
}