use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

use crate::gpu_detect::{detect_gpu_type, get_selected_gpu, GpuType};

pub fn start_stability_test() {
    let selected_gpu = get_selected_gpu();
    let gpu_type = detect_gpu_type();

    thread::spawn(move || {
        let start_time = Instant::now();

        println!(
            "[Stability Test] Starting stability test for {:?} GPU: {}",
            gpu_type, selected_gpu
        );

        // Placeholder command: Replace with actual stability test tool in the future.
        let result = Command::new("bash")
            .arg("-c")
            .arg("sleep 10") // Simulate test duration
            .output();

        match result {
            Ok(_) => {
                let elapsed = start_time.elapsed();
                println!(
                    "[Stability Test] Completed in {:.2?} for GPU: {}",
                    elapsed, selected_gpu
                );
            }
            Err(e) => {
                eprintln!("[Stability Test] Failed to run test: {}", e);
            }
        }
    });
}