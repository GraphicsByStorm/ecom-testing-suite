use once_cell::sync::Lazy;
use std::process::Command;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub enum GpuType {
    AMD,
    Nvidia,
    Unknown,
}

static SELECTED_GPU: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
static GPU_TYPE: Lazy<Mutex<GpuType>> = Lazy::new(|| Mutex::new(GpuType::Unknown));

pub fn detect_gpu_type() -> GpuType {
    let output = Command::new("lspci")
        .arg("-nn")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_lowercase())
        .unwrap_or_else(|_| "unknown".to_string());

    let gpu_type = if output.contains("amd") || output.contains("ati") {
        GpuType::AMD
    } else if output.contains("nvidia") {
        GpuType::Nvidia
    } else {
        GpuType::Unknown
    };

    *GPU_TYPE.lock().unwrap() = gpu_type.clone();
    gpu_type
}

pub fn set_selected_gpu(name: &str) {
    *SELECTED_GPU.lock().unwrap() = name.to_string();
}

pub fn get_selected_gpu() -> String {
    SELECTED_GPU.lock().unwrap().clone()
}