use dotenvy::from_path;
use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

fn main() {
    let home = env::var("HOME").expect("HOME not set");
    let base_path = format!("{}/.config/system_scripts/sunset", home);
    let env_path = format!("{}/.env", base_path);
    let state_path = format!("{}/state", base_path);

    from_path(&env_path).ok();

    let night_temp = env::var("NIGHT_TEMP").expect("NIGHT_TEMP not set in .env");
    let day_temp = env::var("DAY_TEMP").expect("DAY_TEMP not set in .env");

    // Leer temperatura actual guardada
    let current_temp = if Path::new(&state_path).exists() {
        fs::read_to_string(&state_path).unwrap_or_default()
    } else {
        String::new()
    };

    // Decidir siguiente temperatura
    let target_temp = if current_temp.trim() == night_temp {
        day_temp.clone()
    } else {
        night_temp.clone()
    };

    // Guardar nuevo estado
    fs::write(&state_path, &target_temp).ok();

    // Matar instancia previa
    Command::new("pkill")
        .arg("-9")
        .arg("hyprsunset")
        .output()
        .ok();

    // Lanzar nueva instancia
    Command::new("hyprsunset")
        .arg("-t")
        .arg(&target_temp)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start hyprsunset");

    // Notificación
    let mode = if target_temp == night_temp {
        format!("Modo Noche 🌙 ({}K)", night_temp)
    } else {
        format!("Modo Día ☀ ({}K)", day_temp)
    };

    Command::new("notify-send")
        .arg("Hyprsunset")
        .arg(mode)
        .spawn()
        .ok();
}
