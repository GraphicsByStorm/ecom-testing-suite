use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::smart;

pub fn check_disk_select() -> bool {
    smart::check_disk_selection_active()
}

pub fn draw_disk_selection(f: &mut Frame) {
    let drives = smart::get_drive_list();
    let selected_index = smart::get_selected_drive_index();

    let items: Vec<ListItem> = drives
        .iter()
        .map(|drive| ListItem::new(Span::raw(drive.clone())))
        .collect();

    let mut state = ListState::default();
    state.select(Some(selected_index));

    let size = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(size);

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Select Drive for SMART Test"))
        .highlight_style(Style::default().bg(Color::White).fg(Color::Black))
        .highlight_symbol("▶ ");

    let info = Paragraph::new(Span::raw("Use ↑/↓ to navigate, Enter to begin test, q to cancel"))
        .block(Block::default().borders(Borders::ALL).title("Instructions"));

    f.render_stateful_widget(list, chunks[0], &mut state);
    f.render_widget(info, chunks[1]);
}

pub fn exit_disk_selection() {
    smart::exit_disk_selection();
}

pub fn decrement_disk_selection() {
    smart::previous_drive();
}

pub fn increment_disk_selection() {
    smart::next_drive();
}

pub fn run_selected_disk_smart() {
    smart::run_smart_test_on_selected_drive();
}

pub fn draw_smart_output(f: &mut Frame) {
    smart::draw_smart_output(f);
}