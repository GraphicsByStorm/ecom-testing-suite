use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::{
    keyboard_test::{self, check_keyboard_test_active, draw_keyboard_test, exit_keyboard_test},
    gamepad_test::{self, check_gamepad_test_active, draw_gamepad_test, exit_gamepad_test},
    audio_test::{self, check_audio_test_active, draw_audio_test, exit_audio_test},
};

/// Check if any input test is active
pub fn check_input_select() -> bool {
    check_keyboard_test_active() || check_gamepad_test_active() || check_audio_test_active()
}

/// Draw the appropriate input testing screen
pub fn draw_input_selection(f: &mut Frame) {
    if check_keyboard_test_active() {
        draw_keyboard_test(f);
    } else if check_gamepad_test_active() {
        draw_gamepad_test(f);
    } else if check_audio_test_active() {
        draw_audio_test(f);
    }
}

/// Exit input testing mode
pub fn exit_input_selection() {
    if check_keyboard_test_active() {
        exit_keyboard_test();
    } else if check_gamepad_test_active() {
        exit_gamepad_test();
    } else if check_audio_test_active() {
        exit_audio_test();
    }
}

/// Move selection up
pub fn decrement_input_selection() {
    if check_keyboard_test_active() {
        keyboard_test::decrement_device_selection();
    } else if check_gamepad_test_active() {
        gamepad_test::decrement_device_selection();
    } else if check_audio_test_active() {
        audio_test::decrement_device_selection();
    }
}

/// Move selection down
pub fn increment_input_selection() {
    if check_keyboard_test_active() {
        keyboard_test::increment_device_selection();
    } else if check_gamepad_test_active() {
        gamepad_test::increment_device_selection();
    } else if check_audio_test_active() {
        audio_test::increment_device_selection();
    }
}