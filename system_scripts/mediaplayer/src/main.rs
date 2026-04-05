use tokio::process::Command;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use serde::Serialize;

#[derive(Serialize)]
struct SpotifyOutput {
    text: String,
    tooltip: String,
    class: String,
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(1);
    let poll_interval = Duration::from_millis(500);

    // Hilo para monitorear cambios
    tokio::spawn(async move {
        let mut last_combined = String::new();
        
        loop {
            if let Some((artist, title, status)) = get_player_info().await {
                let current = format!("{}|{}|{}", artist, title, status);
                if current != last_combined {
                    let _ = tx.send(build_output(&artist, &title, &status)).await;
                    last_combined = current;
                }
            } else if !last_combined.is_empty() {
                let _ = tx.send(SpotifyOutput {
                    text: "".into(),
                    tooltip: "Spotify no está reproduciendo".into(),
                    class: "stopped".into()
                }).await;
                last_combined.clear();
            }
            sleep(poll_interval).await;
        }
    });

    // Hilo principal para imprimir
    while let Some(output) = rx.recv().await {
        println!("{}", serde_json::to_string(&output).unwrap());
    }
}

async fn get_player_info() -> Option<(String, String, String)> {
    let status = Command::new("playerctl")
        .args(["-p", "spotify", "status"])
        .output()
        .await
        .ok()?;
    
    if !status.status.success() {
        return None;
    }

    let status_str = String::from_utf8(status.stdout).ok()?.trim().to_string();
    if status_str == "Stopped" {
        return None;
    }

    let artist = get_metadata("artist").await?;
    let title = get_metadata("title").await?;

    Some((artist, title, status_str))
}

async fn get_metadata(field: &str) -> Option<String> {
    let output = Command::new("playerctl")
        .args(["-p", "spotify", "metadata", field])
        .output()
        .await
        .ok()?;

    if output.status.success() {
        let value = String::from_utf8(output.stdout).ok()?.trim().to_string();
        if !value.is_empty() { Some(value) } else { None }
    } else {
        None
    }
}

fn build_output(artist: &str, title: &str, status: &str) -> SpotifyOutput {
    SpotifyOutput {
        text: format!(" {} – {}", title, artist),
        tooltip: format!("Spotify ({}): {} – {}", status, artist, title),
        class: status.to_lowercase(),
    }
}