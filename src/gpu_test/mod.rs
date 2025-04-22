use crate::gpu_detect::{get_selected_gpu, detect_gpu_type, GpuType};
use crate::stress_test::start_stress_test;
use crate::stability_test::start_stability_test;

use ratatui::{
    Frame,
    widgets::{Block, Borders, Paragraph},
    text::Span,
};

use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Debug, Clone, Copy)]
pub enum TestMode {
    Stress,
    Stability,
}

pub static GPU_TEST_ACTIVE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
pub static CURRENT_TEST_MODE: Lazy<Mutex<Option<TestMode>>> = Lazy::new(|| Mutex::new(None));
pub static GPU_TEST_STARTED: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

pub fn set_test_mode(mode: TestMode) {
    *GPU_TEST_ACTIVE.lock().unwrap() = true;
    *CURRENT_TEST_MODE.lock().unwrap() = Some(mode);
    *GPU_TEST_STARTED.lock().unwrap() = false; // reset run flag
}

pub fn clear_test_mode() {
    *GPU_TEST_ACTIVE.lock().unwrap() = false;
    *CURRENT_TEST_MODE.lock().unwrap() = None;
    *GPU_TEST_STARTED.lock().unwrap() = false;
}

pub fn check_test_active() -> bool {
    *GPU_TEST_ACTIVE.lock().unwrap()
}

pub fn draw_gpu_testing(f: &mut Frame) {
    let area = f.area();

    let selected_gpu = get_selected_gpu();
    let gpu_type = detect_gpu_type();
    let mode = *CURRENT_TEST_MODE.lock().unwrap();
    let mut started = GPU_TEST_STARTED.lock().unwrap();

    if let Some(test_mode) = mode {
        // Start the test only once
        if !*started {
            match test_mode {
                TestMode::Stress => start_stress_test(),
                TestMode::Stability => start_stability_test(),
            }
            *started = true;
        }

        let status = format!(
            "Running {:?} test on {:?}: {}\n\nPress 'q' to exit...",
            test_mode, gpu_type, selected_gpu
        );

        let block = Block::default().title("GPU Test").borders(Borders::ALL);
        let paragraph = Paragraph::new(Span::raw(status)).block(block);
        f.render_widget(paragraph, area);
    }
}