use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crossterm::event::KeyCode;

use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::photo_exporter;
use crate::smart;
use crate::smart::{enter_disk_selection, scroll_up as smart_scroll_up, scroll_down as smart_scroll_down};
use crate::keyboard_test::enter_keyboard_test;
use crate::gamepad_test::enter_gamepad_test;
use crate::audio_test::enter_audio_test;
use crate::nvidia_drivers::{exit_driver_selection};
use crate::nvidia_drivers;
use crate::gpu_test::{check_test_active, draw_gpu_testing};
use crate::stress_test::{check_stress_active, draw_stress_test_popup, stop_stress_test};
use crate::menu::input::launch_selected_test;

pub mod disk;
pub mod gpu;
pub mod input;

static MENU_OPTIONS: Lazy<Vec<&str>> = Lazy::new(|| {
    vec![
        "Run SMART Test",              // 0
        "AMD GPU Test",                // 1
        "NVIDIA GPU Test",            // 2
        "Photo Exporter",             // 3
        "NVIDIA Driver Installer",    // 4
        "Keyboard Test",              // 5
        "Gamepad Test",               // 6
        "Audio Test",                 // 7
        "Exit",                       // 8
    ]
});

static MENU_INDEX: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(0));

pub fn draw_main_menu(f: &mut Frame) {
    let area = f.area();
    let selected = *MENU_INDEX.lock().unwrap();

    let mut lines = Vec::new();
    for (i, option) in MENU_OPTIONS.iter().enumerate() {
        let prefix = if i == selected { "▶ " } else { "  " };

        let style = if i == selected {
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        lines.push(Line::from(Span::styled(format!("{}{}", prefix, option), style)));
    }

    let paragraph = Paragraph::new(Text::from(lines))
        .block(Block::default().title("Main Menu").borders(Borders::ALL))
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}

pub fn increment_menu() {
    let mut index = MENU_INDEX.lock().unwrap();
    if *index < MENU_OPTIONS.len().saturating_sub(1) {
        *index += 1;
    }
}

pub fn decrement_menu() {
    let mut index = MENU_INDEX.lock().unwrap();
    if *index > 0 {
        *index -= 1;
    }
}

pub fn handle_main_menu_enter() {
    let index = *MENU_INDEX.lock().unwrap();
    match index {
        0 => enter_disk_selection(),
        1 => gpu::enter_driver_selection(),
        2 => gpu::enter_driver_selection(),
        3 => photo_exporter::run_photo_exporter(),
        4 => nvidia_drivers::enter_driver_selection(),
        5 => enter_keyboard_test(),
        6 => enter_gamepad_test(),
        7 => enter_audio_test(),
        8 => std::process::exit(0),
        _ => {}
    }
}

pub fn draw_conditional_screens(f: &mut Frame) {
    if smart::check_smart_active() {
        smart::draw_smart_output(f);
    } else if check_test_active() {
        draw_gpu_testing(f);
    } else if check_stress_active() {
        draw_stress_test_popup(f);
    } else if disk::check_disk_select() {
        disk::draw_disk_selection(f);
    } else if input::check_input_select() {
        input::draw_input_selection(f);
    } else if nvidia_drivers::check_driver_selection() {
        gpu::draw_driver_menu(f);
    } else if photo_exporter::check_export_active() {
        photo_exporter::draw_photo_export_progress(f);
    } else {
        draw_main_menu(f);
    }
}

pub fn handle_key_press(key: KeyCode) {
    match key {
        KeyCode::Char('q') => {
            if smart::check_smart_active() {
                smart::exit_smart_output();
            } else if disk::check_disk_select() {
                disk::exit_disk_selection();
            } else if input::check_input_select() {
                input::exit_input_selection();
            } else if nvidia_drivers::check_driver_selection() {
                exit_driver_selection();
            } else if photo_exporter::check_export_active() {
                photo_exporter::exit_export();
            } else if check_stress_active() {
                stop_stress_test();
            } else {
                std::process::exit(0);
            }
        }
        KeyCode::Up => {
            if smart::check_smart_active() {
                smart_scroll_up();
            } else if disk::check_disk_select() {
                disk::decrement_disk_selection();
            } else if input::check_input_select() {
                input::decrement_input_selection();
            } else if nvidia_drivers::check_driver_selection() {
                gpu::decrement_driver_selection_menu();
            } else {
                decrement_menu();
            }
        }
        KeyCode::Down => {
            if smart::check_smart_active() {
                smart_scroll_down();
            } else if disk::check_disk_select() {
                disk::increment_disk_selection();
            } else if input::check_input_select() {
                input::increment_input_selection();
            } else if nvidia_drivers::check_driver_selection() {
                gpu::increment_driver_selection_menu();
            } else {
                increment_menu();
            }
        }
        KeyCode::Enter => {
            if disk::check_disk_select() {
                disk::run_selected_disk_smart();
            } else if input::check_input_select() {
                launch_selected_test();
            } else if nvidia_drivers::check_driver_selection() {
                gpu::install_selected_driver_menu();
            } else {
                handle_main_menu_enter();
            }
        }
        _ => {}
    }
}