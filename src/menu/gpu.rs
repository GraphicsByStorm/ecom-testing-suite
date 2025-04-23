use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::nvidia_drivers::{
    get_driver_list,
    get_driver_index,
    check_driver_selection,
    check_driver_installing,
    draw_driver_install_output,
    install_selected_driver,
    reset_driver_state,
    increment_driver_selection,
    decrement_driver_selection,
};

pub static DRIVER_LIST: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(vec![]));

pub fn enter_driver_selection() {
    *DRIVER_LIST.lock().unwrap() = get_driver_list();
}

pub fn check_driver_select() -> bool {
    check_driver_selection()
}

pub fn draw_driver_menu(f: &mut Frame) {
    if check_driver_installing() {
        draw_driver_install_output(f);
        return;
    }

    let drivers = DRIVER_LIST.lock().unwrap();
    let index = get_driver_index();

    let items: Vec<ListItem> = drivers
        .iter()
        .map(|d| ListItem::new(Span::raw(d.clone())))
        .collect();

    let mut state = ListState::default();
    state.select(Some(index));

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(f.area());

    let list = List::new(items)
        .block(Block::default().title("Select NVIDIA Driver").borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
        .highlight_symbol("▶ ");

    let info = Paragraph::new(Span::raw(
        "Use ↑/↓ to choose driver. Press Enter to install. Press q to cancel.",
    ))
    .block(Block::default().borders(Borders::ALL).title("Instructions"));

    f.render_stateful_widget(list, layout[0], &mut state);
    f.render_widget(info, layout[1]);
}

pub fn increment_driver_selection_menu() {
    increment_driver_selection();
}

pub fn decrement_driver_selection_menu() {
    decrement_driver_selection();
}

pub fn install_selected_driver_menu() {
    install_selected_driver();
}

pub fn exit_driver_selection_menu() {
    reset_driver_state();
}