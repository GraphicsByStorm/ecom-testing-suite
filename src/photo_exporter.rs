use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::{Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};
use std::{process::Command, sync::Mutex, thread, time::Duration};
use once_cell::sync::Lazy;

pub static EXPORT_ACTIVE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
pub static EXPORT_PROGRESS: Lazy<Mutex<u16>> = Lazy::new(|| Mutex::new(0));
pub static EXPORT_MESSAGE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

pub fn check_export_active() -> bool {
    *EXPORT_ACTIVE.lock().unwrap()
}

pub fn exit_export() {
    *EXPORT_ACTIVE.lock().unwrap() = false;
    *EXPORT_PROGRESS.lock().unwrap() = 0;
    *EXPORT_MESSAGE.lock().unwrap() = String::new();
}

pub fn run_photo_exporter() {
    *EXPORT_ACTIVE.lock().unwrap() = true;
    *EXPORT_PROGRESS.lock().unwrap() = 0;
    *EXPORT_MESSAGE.lock().unwrap() = "Preparing to export photos...".to_string();

    thread::spawn(|| {
        // Navigate to export directory
        let base_path = "/home/ecom/Pictures/ebay";
        let default_start = 84;

        let output = Command::new("bash")
            .arg("-c")
            .arg(format!(
                r#"
cd "{0}" || exit 1
last_num=$(ls -d SW* 2>/dev/null | grep -E '^SW[0-9]{{3}}$' | sed 's/SW//' | sort -n | tail -n 1)
if [[ -z "$last_num" ]]; then
  next_num={1}
else
  next_num=$((10#$last_num + 1))
fi
if (( next_num >= 100 )); then
  new_folder="SW$next_num"
else
  new_folder=$(printf "SW%03d" "$next_num")
fi
mkdir "$new_folder" && cd "$new_folder" || exit 1
gphoto2 --get-all-files
cd ..
                "#, base_path, default_start
            ))
            .output();

        for i in 0..=100 {
            *EXPORT_PROGRESS.lock().unwrap() = i;
            thread::sleep(Duration::from_millis(20));
        }

        match output {
            Ok(out) => {
                *EXPORT_MESSAGE.lock().unwrap() = format!(
                    "Photo export complete.\n{}",
                    String::from_utf8_lossy(&out.stdout)
                );
            }
            Err(e) => {
                *EXPORT_MESSAGE.lock().unwrap() = format!("Photo export failed: {}", e);
            }
        }
    });
}

pub fn draw_photo_export_progress(f: &mut Frame) {
    let size = f.area();
    let progress = *EXPORT_PROGRESS.lock().unwrap();
    let message = EXPORT_MESSAGE.lock().unwrap();

    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .margin(2)
        .split(size);

    let gauge = Gauge::default()
        .block(Block::default().title("Export Progress").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Magenta).bg(Color::Black))
        .percent(progress);

    let paragraph = Paragraph::new(Span::raw(message.as_str()))
        .block(Block::default().borders(Borders::ALL).title("Status"));

    f.render_widget(gauge, chunks[0]);
    f.render_widget(paragraph, chunks[1]);
}