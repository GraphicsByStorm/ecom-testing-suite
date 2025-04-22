use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

use crate::gpu_detect::{detect_gpu_type, get_selected_gpu, GpuType};

static HEAVEN_PATH: &str = "/opt/unigine-heaven/bin/heaven_x64";

pub fn start_stress_test() {
    let selected_gpu = get_selected_gpu();
    let gpu_type = detect_gpu_type();

    thread::spawn(move || {
        let start_time = Instant::now();

        println!(
            "[Stress Test] Starting Unigine Heaven for {:?} GPU: {}",
            gpu_type, selected_gpu
        );

        let heaven_args = get_optimal_args_for_gpu(&gpu_type);

        match Command::new(HEAVEN_PATH).args(&heaven_args).spawn() {
            Ok(mut child) => {
                let _ = child.wait();
                let elapsed = start_time.elapsed();
                println!(
                    "[Stress Test] Completed in {:.2?} for GPU: {}",
                    elapsed, selected_gpu
                );
            }
            Err(e) => {
                eprintln!("[Stress Test] Failed to start Heaven benchmark: {}", e);
            }
        }
    });
}

fn get_optimal_args_for_gpu(gpu_type: &GpuType) -> Vec<&'static str> {
    match gpu_type {
        GpuType::Nvidia => vec![
            "-video_app", "opengl",
            "-video_mode", "1920x1080",
            "-sound", "0",
            "-fullscreen", "0",
            "-preset", "extreme",
        ],
        GpuType::AMD => vec![
            "-video_app", "vulkan",
            "-video_mode", "1920x1080",
            "-sound", "0",
            "-fullscreen", "0",
            "-preset", "extreme",
        ],
        GpuType::Unknown => vec![
            "-video_app", "opengl",
            "-video_mode", "1280x720",
            "-sound", "0",
            "-fullscreen", "0",
            "-preset", "basic",
        ],
    }
}