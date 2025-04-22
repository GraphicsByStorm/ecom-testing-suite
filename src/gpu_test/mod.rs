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

#[derive(Clone, Copy)]
pub enum TestMode {
    Stress,
    Stability,
}

pub static GPU_TEST_ACTIVE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
pub static CURRENT_TEST_MODE: Lazy<Mutex<Option<TestMode>>> = Lazy::new(|| Mutex::new(None));

pub fn set_test_mode(mode: TestMode) {
    *GPU_TEST_ACTIVE.lock().unwrap() = true;
    *CURRENT_TEST_MODE.lock().unwrap() = Some(mode);
}

pub fn clear_test_mode() {
    *GPU_TEST_ACTIVE.lock().unwrap() = false;
    *CURRENT_TEST_MODE.lock().unwrap() = None;
}

pub fn check_test_active() -> bool {
    *GPU_TEST_ACTIVE.lock().unwrap()
}

pub fn draw_gpu_testing(f: &mut Frame) {
    let area = f.area();

    let selected_gpu = get_selected_gpu();
    let gpu_type = detect_gpu_type();
    let mode = *CURRENT_TEST_MODE.lock().unwrap();

    if let Some(test_mode) = mode {
        match test_mode {
            TestMode::Stress => {
                start_stress_test(&selected_gpu, &gpu_type);
            }
            TestMode::Stability => {
                start_stability_test(&selected_gpu, &gpu_type);
            }
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