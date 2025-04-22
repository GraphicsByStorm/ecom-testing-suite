use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::smart::enter_disk_selection;

pub mod disk;
pub mod gpu;
pub mod input;

pub static MENU_INDEX: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

pub static MENU_OPTIONS: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "SMART Test",
        "AMD GPU Test",
        "NVIDIA GPU Test",
        "Photo Exporter",
        "Install NVIDIA Drivers",
        "Keyboard Test",
        "Gamepad Test",
        "Audio Test",
        "Exit",
    ]
});

pub fn draw_main_menu(f: &mut Frame) {
    let size = f.area();
    let menu_items: Vec<ListItem> = MENU_OPTIONS
        .iter()
        .map(|item| ListItem::new(Line::from(Span::raw(*item))))
        .collect();

    let mut state = ListState::default();
    let index = *MENU_INDEX.lock().unwrap();
    state.select(Some(index));

    let list = List::new(menu_items)
        .block(Block::default().borders(Borders::ALL).title(" Main Menu "))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
        .highlight_symbol("▶ ");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Min(1)])
        .split(size);

    f.render_stateful_widget(list, chunks[0], &mut state);
}

pub fn increment_menu() {
    let mut index = MENU_INDEX.lock().unwrap();
    *index = (*index + 1) % MENU_OPTIONS.len();
}

pub fn decrement_menu() {
    let mut index = MENU_INDEX.lock().unwrap();
    if *index == 0 {
        *index = MENU_OPTIONS.len() - 1;
    } else {
        *index -= 1;
    }
}

pub fn get_selected_menu_index() -> usize {
    *MENU_INDEX.lock().unwrap()
}

pub fn handle_main_menu_enter() {
    match get_selected_menu_index() {
        0 => disk::enter_disk_selection(),
        1 => gpu::enter_amd_gpu_selection(),
        2 => gpu::enter_nvidia_gpu_selection(),
        3 => crate::photo_exporter::run_photo_exporter(),
        4 => gpu::enter_driver_selection(),
        5 => crate::keyboard_test::enter_keyboard_test(),
        6 => crate::gamepad_test::enter_gamepad_test(),
        7 => crate::audio_test::enter_audio_test(),
        8 => std::process::exit(0),
        _ => {}
    }
}