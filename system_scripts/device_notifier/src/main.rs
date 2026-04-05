use notify_rust::Notification;
use std::collections::HashMap;
use std::io;
use std::os::unix::io::AsRawFd;
use udev::{Device, EventType, MonitorBuilder};

fn get_clean_prop(dev: &Device, prop: &str) -> Option<String> {
    dev.property_value(prop)
        .and_then(|v| v.to_str())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && s != "Unknown" && s != "0000")
}

fn main() -> io::Result<()> {
    let socket = MonitorBuilder::new()?.match_subsystem("usb")?.listen()?;
    let fd = socket.as_raw_fd();

    // Mapa para recordar qué dispositivo está en cada puerto
    // Key: SysPath (único), Value: Nombre amigable
    let mut device_cache: HashMap<std::path::PathBuf, String> = HashMap::new();

    println!("ArchWatcher: Monitor de estado con Cache activo... 🚀");

    loop {
        unsafe {
            let mut fds = [libc::pollfd {
                fd,
                events: libc::POLLIN,
                revents: 0,
            }];
            if libc::poll(fds.as_mut_ptr(), 1, -1) > 0 {
                if let Some(event) = socket.iter().next() {
                    let dev = event.device();
                    let syspath = dev.syspath().to_path_buf();

                    let is_usb_device = dev
                        .property_value("DEVTYPE")
                        .map_or(false, |t| t == "usb_device");

                    if !is_usb_device {
                        continue;
                    }

                    match event.event_type() {
                        EventType::Add => {
                            let vendor = get_clean_prop(&dev, "ID_VENDOR_FROM_DATABASE")
                                .or_else(|| get_clean_prop(&dev, "ID_VENDOR"))
                                .unwrap_or_else(|| "Fabricante".into());

                            let model = get_clean_prop(&dev, "ID_MODEL_FROM_DATABASE")
                                .or_else(|| get_clean_prop(&dev, "ID_MODEL"))
                                .unwrap_or_else(|| "Genérico".into());

                            let full_name = format!("{} {}", vendor, model);

                            // Guardamos en cache antes de notificar
                            device_cache.insert(syspath, full_name.clone());

                            Notification::new()
                                .summary("🔌 USB Conectado")
                                .body(&format!("<b>Hardware:</b> {}", full_name))
                                .appname("ArchWatcher")
                                .icon("drive-removable-media-usb")
                                .timeout(5000)
                                .show()
                                .ok();
                        }
                        EventType::Remove => {
                            // Recuperamos el nombre del cache usando el syspath
                            let device_name = device_cache
                                .remove(&syspath)
                                .unwrap_or_else(|| "Dispositivo desconocido".into());

                            Notification::new()
                                .summary("🚫 USB Desconectado")
                                .body(&format!("Se ha retirado: <b>{}</b>", device_name))
                                .appname("ArchWatcher")
                                .icon("media-eject")
                                .timeout(3000)
                                .show()
                                .ok();
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
