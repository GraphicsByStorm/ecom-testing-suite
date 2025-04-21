// Required imports
use std::{io, thread};
use std::process::Command;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::event::{self, Event, KeyCode, EnableMouseCapture, DisableMouseCapture};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

mod smart;
mod amd_gpu;
mod photo_exporter;

// Module imports
use smart::{
    check_smart_active, draw_smart_output, exit_smart_output, check_smart_disk_select,
    draw_disk_selection, exit_disk_selection, enter_disk_selection, increment_disk_selection,
    decrement_disk_selection, scroll_output_down as scroll_smart_down,
    scroll_output_up as scroll_smart_up, run_selected_smart
};
use amd_gpu::{
    check_amd_active, draw_amd_output, exit_amd_output, run_amd_gpu_check,
    check_gpu_select, draw_gpu_selection, enter_gpu_selection, exit_gpu_selection,
    increment_gpu_selection, decrement_gpu_selection, run_selected_gpu_check,
    scroll_output_down as scroll_amd_down, scroll_output_up as scroll_amd_up
};
use photo_exporter::{
    run_photo_exporter, check_export_active, draw_photo_export_progress, exit_export
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal);

    disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    result
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let menu_items = vec!["SMART Test", "AMD GPU Test", "Photo Export"];
    let mut selected_menu_index = 0;

    loop {
        terminal.draw(|f| {
            let size = f.area();

            if check_smart_active() {
                draw_smart_output(f);
            } else if check_smart_disk_select() {
                draw_disk_selection(f);
            } else if check_amd_active() {
                draw_amd_output(f);
            } else if check_gpu_select() {
                draw_gpu_selection(f);
            } else if check_export_active() {
                draw_photo_export_progress(f);
            } else {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(5)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Min(1),
                    ])
                    .split(size);

                let menu: Vec<ListItem> = menu_items
                    .iter()
                    .map(|i| ListItem::new(Line::from(*i)))
                    .collect();

                let mut state = ListState::default();
                state.select(Some(selected_menu_index));

                let list = List::new(menu)
                    .block(Block::default().title(" Main Menu ").borders(Borders::ALL))
                    .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
                    .highlight_symbol("▶ ");

                f.render_stateful_widget(list, chunks[1], &mut state);
            }
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
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
                        } else if check_export_active() {
                            exit_export();
                        } else {
                            break;
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
                    KeyCode::Enter => {
                        if check_smart_disk_select() {
                            run_selected_smart();
                        } else if check_gpu_select() {
                            run_selected_gpu_check();
                        } else {
                            match selected_menu_index {
                                0 => enter_disk_selection(),
                                1 => enter_gpu_selection(),
                                2 => run_photo_exporter(),
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