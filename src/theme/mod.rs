use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders},
};

/// Returns a reusable bordered block with the given title.
pub fn bordered_block(title: &str) -> Block {
    Block::default()
        .title(Span::styled(
            title,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
}

/// Returns a reusable highlight style for selected list items.
pub fn highlight_style() -> Style {
    Style::default()
        .fg(Color::Black)
        .bg(Color::White)
        .add_modifier(Modifier::BOLD)
}

/// Returns an info box Line with formatted content.
pub fn info_box<'a>(title: &'a str, content: &'a str) -> Line<'a> {
    Line::from(vec![
        Span::styled(title, Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
        Span::raw(": "),
        Span::raw(content),
    ])
}