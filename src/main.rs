use std::{io, time::Duration};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

mod menu;
mod theme;
mod smart;
mod photo_exporter;
mod nvidia_drivers;
mod keyboard_test;
mod gamepad_test;
mod audio_test;
mod gpu_test;
mod stress_test;
mod stability_test;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    result
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            if menu::disk::check_disk_select() {
                menu::disk::draw_disk_selection(f);
            } else if menu::input::check_input_select() {
                menu::input::draw_input_selection(f);
            } else if menu::gpu::check_driver_select() {
                menu::gpu::draw_driver_menu(f);
            } else if nvidia_drivers::check_driver_installing() {
                nvidia_drivers::draw_driver_install_output(f);
            } else if smart::check_smart_active() {
                menu::disk::draw_smart_output(f);
            } else if photo_exporter::check_export_active() {
                photo_exporter::draw_photo_export_progress(f);
            } else if stress_test::check_stress_active() {
                stress_test::draw_stress_test_popup(f);
            } else {
                menu::draw_main_menu(f);
            }
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        if smart::check_smart_active() {
                            smart::exit_smart_output();
                        } else if menu::disk::check_disk_select() {
                            menu::disk::exit_disk_selection();
                        } else if menu::input::check_input_select() {
                            menu::input::exit_input_selection();
                        } else if menu::gpu::check_driver_select() {
                            menu::gpu::exit_driver_selection_menu();
                        } else if photo_exporter::check_export_active() {
                            photo_exporter::exit_export();
                        } else if stress_test::check_stress_active() {
                            stress_test::stop_stress_test();
                        } else {
                            break;
                        }
                    }
                    KeyCode::Up => {
                        if menu::disk::check_disk_select() {
                            menu::disk::decrement_disk_selection();
                        } else if menu::input::check_input_select() {
                            menu::input::decrement_input_selection();
                        } else if menu::gpu::check_driver_select() {
                            menu::gpu::decrement_driver_selection_menu();
                        } else {
                            menu::decrement_menu();
                        }
                    }
                    KeyCode::Down => {
                        if menu::disk::check_disk_select() {
                            menu::disk::increment_disk_selection();
                        } else if menu::input::check_input_select() {
                            menu::input::increment_input_selection();
                        } else if menu::gpu::check_driver_select() {
                            menu::gpu::increment_driver_selection_menu();
                        } else {
                            menu::increment_menu();
                        }
                    }
                    KeyCode::Enter => {
                        if menu::disk::check_disk_select() {
                            menu::disk::run_selected_disk_smart();
                        } else if menu::input::check_input_select() {
                            // Input selection logic will go here.
                        } else if menu::gpu::check_driver_select() {
                            menu::gpu::install_selected_driver_menu();
                        } else {
                            menu::handle_main_menu_enter();
                        }
                    }
                    KeyCode::Char('s') => {
                        stress_test::start_stress_test();
                    }
                    KeyCode::Char('t') => {
                        stability_test::start_stability_test();
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}