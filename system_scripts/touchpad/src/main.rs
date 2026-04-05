use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Obtener la ruta del ejecutable
    let exe_path = env::current_exe().expect("No se pudo obtener la ruta del ejecutable");

    // Directorio del ejecutable
    let dir_path = exe_path.parent().expect("No se pudo obtener el directorio del ejecutable");

    // Ruta del archivo de estado
    let state_file: PathBuf = dir_path.join("touchpad_status");

    // Leer estado
    let is_enabled = match fs::read_to_string(&state_file) {
        Ok(content) => content.trim() == "enabled",
        Err(_) => true, // Asume activado si no existe
    };

    // Nuevo estado
    let (new_state, notify_msg, state_string) = if is_enabled {
        ("false", "Touchpad desactivado ❌", "disabled")
    } else {
        ("true", "Touchpad activado ✅", "enabled")
    };

    // Aplicar a ambos touchpads
    let devices = [
        "device[elan071a:00-04f3:30fd-touchpad]:enabled",
        "device[etps/2-elantech-touchpad]:enabled",
    ];

    for dev in devices {
        let _ = Command::new("hyprctl")
            .arg("keyword")
            .arg(dev)
            .arg(new_state)
            .status();
    }

    // Guardar nuevo estado
    let _ = fs::write(&state_file, state_string);

    // Notificación
    let _ = Command::new("notify-send")
        .arg("Touchpad")
        .arg(notify_msg)
        .status();
}
