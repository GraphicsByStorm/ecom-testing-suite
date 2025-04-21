use std::{io, time::Duration};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{List, ListItem},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color},
    text::Span,
};

mod smart;
mod theme;
mod amd_gpu;

use crate::theme::*;
use crate::smart::{
    check_smart_active, exit_smart_output, scroll_output_down as scroll_smart_down,
    scroll_output_up as scroll_smart_up, run_selected_smart, enter_disk_selection,
    check_smart_disk_select, exit_disk_selection, increment_disk_selection, decrement_disk_selection,
    draw_smart_output, draw_disk_selection,
};

use crate::amd_gpu::{
    draw_amd_output, draw_gpu_selection, check_amd_active, check_gpu_select,
    exit_amd_output, exit_gpu_selection, run_selected_gpu_check,
    increment_gpu_selection, decrement_gpu_selection, enter_gpu_selection,
    scroll_output_down as scroll_amd_down,
    scroll_output_up as scroll_amd_up,
};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> io::Result<()> {
    let mut selected_menu_index = 0;
    let menu_items = vec!["SMART Disk Test", "AMD GPU Test"];

    loop {
        terminal.draw(|f| {
            let size = f.size();

            if check_smart_active() {
                draw_smart_output(f);
            } else if check_smart_disk_select() {
                draw_disk_selection(f);
            } else if check_amd_active() {
                draw_amd_output(f);
            } else if check_gpu_select() {
                draw_gpu_selection(f);
            } else {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(5)
                    .constraints(
                        [
                            Constraint::Percentage(20),
                            Constraint::Percentage(80),
                        ]
                        .as_ref(),
                    )
                    .split(size);

                let title = List::new(vec![ListItem::new(Span::styled(
                    "Electronics Testing Suite",
                    Style::default().fg(Color::Cyan),
                ))])
                .block(menu_block(" ELECTRONICS TESTING SUITE "));

                let items: Vec<ListItem> = menu_items
                    .iter()
                    .enumerate()
                    .map(|(_, m)| ListItem::new(Span::styled(*m, normal_text_style())))
                    .collect();

                let menu = List::new(items)
                    .block(menu_block(" SELECT A TEST "))
                    .highlight_style(highlight_style())
                    .highlight_symbol(">> ");

                f.render_widget(title, chunks[0]);
                let mut state = ratatui::widgets::ListState::default();
                state.select(Some(selected_menu_index));
                f.render_stateful_widget(menu, chunks[1], &mut state);
            }
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        if check_smart_active() {
                            exit_smart_output();
                        } else if check_smart_disk_select() {
                            exit_disk_selection();
                        } else if check_amd_active() {
                            exit_amd_output();
                        } else if check_gpu_select() {
                            exit_gpu_selection();
                        } else {
                            break;
                        }
                    }
                    KeyCode::Up => {
                        if check_smart_disk_select() {
                            decrement_disk_selection();
                        } else if check_gpu_select() {
                            decrement_gpu_selection();
                        } else {
                            selected_menu_index = selected_menu_index.saturating_sub(1);
                        }
                    }
                    KeyCode::Down => {
                        if check_smart_disk_select() {
                            increment_disk_selection();
                        } else if check_gpu_select() {
                            increment_gpu_selection();
                        } else {
                            selected_menu_index = (selected_menu_index + 1) % menu_items.len();
                        }
                    }
                    KeyCode::Char('j') => {
                        if check_smart_active() {
                            scroll_smart_down();
                        } else if check_amd_active() {
                            scroll_amd_down();
                        }
                    }
                    KeyCode::Char('k') => {
                        if check_smart_active() {
                            scroll_smart_up();
                        } else if check_amd_active() {
                            scroll_amd_up();
                        }
                    }
                    KeyCode::Enter => {
                        if check_smart_disk_select() {
                            run_selected_smart();
                        } else if check_gpu_select() {
                            run_selected_gpu_check();
                        } else {
                            match selected_menu_index {
                                0 => enter_disk_selection(),
                                1 => enter_gpu_selection(),
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}