use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders};

pub fn default_block(title: &str) -> Block {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Gray))
}

pub fn highlight_style() -> Style {
    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
}

pub fn health_status_style(health: &str) -> Style {
    match health {
        "GREAT" => Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD),
        "GOOD" => Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD),
        "BAD" => Style::default().fg(Color::Black).bg(Color::Red).add_modifier(Modifier::BOLD),
        _ => Style::default().fg(Color::White).bg(Color::DarkGray),
    }
}

pub fn menu_block(title: &str) -> Block {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
}

pub fn list_highlight() -> Style {
    Style::default().bg(Color::Blue)
}

pub fn normal_text_style() -> Style {
    Style::default()
}