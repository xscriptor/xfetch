use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
struct PluginRequest {
    version: Option<u32>,
    kind: Option<String>,
    lines: Vec<String>,
    args: Option<PluginArgs>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
struct PluginArgs {
    fps: Option<u64>,
    duration_ms: Option<u64>,
    #[serde(rename = "loop")]
    loop_enabled: Option<bool>,
}

#[derive(Debug, Serialize)]
struct PluginResponse {
    frames: Vec<Frame>,
}

#[derive(Debug, Serialize)]
struct Frame {
    delay_ms: u64,
    lines: Vec<String>,
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let request: PluginRequest = match serde_json::from_str(&input) {
        Ok(value) => value,
        Err(_) => return,
    };

    let args = request.args.unwrap_or_default();
    let fps = clamp(args.fps.unwrap_or(12), 1, 60);
    let frame_delay = 1000 / fps;
    let duration_ms = std::cmp::max(frame_delay, args.duration_ms.unwrap_or(1200));
    let frame_count = std::cmp::max(1, duration_ms / frame_delay);

    let mut frames = Vec::new();
    for index in 0..frame_count {
        let offset = index as usize;
        let mut lines = Vec::new();
        for line in &request.lines {
            lines.push(color_sweep(line, offset));
        }
        frames.push(Frame {
            delay_ms: frame_delay,
            lines,
        });
    }

    let response = PluginResponse { frames };
    if let Ok(body) = serde_json::to_string(&response) {
        let _ = io::stdout().write_all(body.as_bytes());
    }
}

fn color_sweep(line: &str, offset: usize) -> String {
    let palette = [31, 32, 33, 34, 35, 36];
    let mut out = String::new();

    for (idx, ch) in line.chars().enumerate() {
        if ch == ' ' {
            out.push(' ');
            continue;
        }
        let color = palette[(idx + offset) % palette.len()];
        out.push_str(&format!("\u001b[{}m{}\u001b[0m", color, ch));
    }

    out
}

fn clamp(value: u64, min: u64, max: u64) -> u64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
