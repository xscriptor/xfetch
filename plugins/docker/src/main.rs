use serde::Deserialize;
use serde::Serialize;
use std::io::{self, Read, Write};
use std::process::Command;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct PluginRequest {
    version: Option<u32>,
    kind: Option<String>,
    args: Option<PluginArgs>,
}

#[derive(Debug, Deserialize)]
struct PluginArgs {}

#[derive(Debug, Serialize)]
struct PluginResponse {
    lines: Vec<String>,
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let _request: PluginRequest = match serde_json::from_str(&input) {
        Ok(value) => value,
        Err(_) => return,
    };

    let lines = get_docker_info();

    let response = PluginResponse { lines };
    if let Ok(body) = serde_json::to_string(&response) {
        let _ = io::stdout().write_all(body.as_bytes());
    }
}

fn get_docker_info() -> Vec<String> {
    let mut result = Vec::new();

    let info_output = Command::new("docker")
        .args(["info", "--format", "{{.Containers}} {{.ContainersRunning}} {{.ContainersPaused}} {{.ContainersStopped}}"])
        .output();

    match info_output {
        Ok(output) if output.status.success() => {
            let stats = String::from_utf8_lossy(&output.stdout);
            let parts: Vec<&str> = stats.trim().split_whitespace().collect();
            if parts.len() >= 4 {
                let total = parts[0];
                let running = parts[1];
                let paused = parts[2];
                let stopped = parts[3];
                result.push(format!(" Containers: {} total", total));
                result.push(format!("  ▶ {} running", running));
                result.push(format!("  ⏸ {} paused", paused));
                result.push(format!("  ⏹ {} stopped", stopped));
            }
        }
        Ok(_) => result.push(" Docker: daemon not running".to_string()),
        Err(_) => result.push(" Docker: not found".to_string()),
    }

    result
}
