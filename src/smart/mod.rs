use std::{process::Command, sync::Mutex};
use once_cell::sync::Lazy;

static SMART_ACTIVE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
static SMART_OUTPUT: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

pub fn check_smart_active() -> bool {
    *SMART_ACTIVE.lock().unwrap()
}

pub fn exit_smart_output() {
    *SMART_ACTIVE.lock().unwrap() = false;
    *SMART_OUTPUT.lock().unwrap() = String::new();
}

pub fn run_smart_test(device: &str) {
    *SMART_ACTIVE.lock().unwrap() = true;

    let output = Command::new("smartctl")
        .arg("-a")
        .arg(device)
        .output();

    match output {
        Ok(result) => {
            let parsed = String::from_utf8_lossy(&result.stdout).to_string();
            *SMART_OUTPUT.lock().unwrap() = parsed;
        }
        Err(e) => {
            *SMART_OUTPUT.lock().unwrap() = format!("Failed to run SMART test: {}", e);
        }
    }
}

pub fn get_smart_output() -> String {
    SMART_OUTPUT.lock().unwrap().clone()
}