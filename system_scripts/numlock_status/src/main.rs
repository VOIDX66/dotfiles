use std::fs;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use serde::Serialize;

#[derive(Serialize)]
struct WaybarOutput {
    text: String,
    tooltip: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    class: Option<String>,
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(true, Ordering::SeqCst);
    }).expect("Error setting signal handler");

    const ICON: &str = "󰎠";
    const POLL_INTERVAL_MS: u64 = 250;

    let led_path = match find_numlock_led() {
        Some(path) => path,
        None => {
            println!("{}", serde_json::to_string(&WaybarOutput {
                text: "".to_string(),
                tooltip: "NumLock LED no encontrado".to_string(),
                class: None
            }).unwrap());
            return;
        }
    };

    let mut last_state = read_numlock_state(&led_path);
    print_output(last_state, ICON);

    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(POLL_INTERVAL_MS));
        
        let current_state = read_numlock_state(&led_path);
        if current_state != last_state {
            print_output(current_state, ICON);
            last_state = current_state;
        }
    }
}

fn print_output(state: bool, icon: &str) {
    let output = WaybarOutput {
        text: if state { icon.to_string() } else { "".to_string() },
        tooltip: if state { "NumLock: ON".to_string() } else { "NumLock: OFF".to_string() },
        class: Some(if state { "enabled".to_string() } else { "disabled".to_string() })
    };
    
    println!("{}", serde_json::to_string(&output).unwrap());
}

fn find_numlock_led() -> Option<String> {
    let leds_dir = "/sys/class/leds";
    let entries = fs::read_dir(leds_dir).ok()?;
    
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.to_str()?.contains("::numlock") {
                return Some(path.join("brightness").to_str()?.to_string());
            }
        }
    }
    None
}

fn read_numlock_state(path: &str) -> bool {
    fs::read_to_string(path)
        .map(|content| content.trim() == "1")
        .unwrap_or(false)
}