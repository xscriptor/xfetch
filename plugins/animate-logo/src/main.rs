use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
struct PluginRequest {
    version: Option<u32>,
    kind: Option<String>,
    lines: Vec<String>,
    frames: Option<Vec<Vec<String>>>,
    args: Option<PluginArgs>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
struct PluginArgs {
    fps: Option<u64>,
    duration_ms: Option<u64>,
    #[serde(rename = "loop")]
    loop_enabled: Option<bool>,
    style: Option<String>,
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
    let style = args.style.as_deref().unwrap_or("sweep");
    let frame_sets = request.frames.unwrap_or_default();

    let frames: Vec<Frame> = match style {
        "frame" if !frame_sets.is_empty() => {
            generate_ascii_frame_animation(&frame_sets, frame_count, frame_delay)
        }
        "wave" => generate_wave_animation(&request.lines, frame_count, frame_delay),
        "rainbow" => generate_rainbow_animation(&request.lines, frame_count, frame_delay),
        "sparkle" => generate_sparkle_animation(&request.lines, frame_count, frame_delay),
        "breathing" => generate_breathing_animation(&request.lines, frame_count, frame_delay),
        "none" => generate_static_animation(&request.lines, frame_count, frame_delay),
        _ => generate_sweep_animation(&request.lines, frame_count, frame_delay),
    };

    let response = PluginResponse { frames };
    if let Ok(body) = serde_json::to_string(&response) {
        let _ = io::stdout().write_all(body.as_bytes());
    }
}

fn generate_sweep_animation(lines: &[String], count: u64, delay: u64) -> Vec<Frame> {
    (0..count)
        .map(|i| Frame {
            delay_ms: delay,
            lines: lines.iter().map(|l| color_sweep(l, i as usize)).collect(),
        })
        .collect()
}

fn generate_wave_animation(lines: &[String], count: u64, delay: u64) -> Vec<Frame> {
    (0..count)
        .map(|i| Frame {
            delay_ms: delay,
            lines: lines.iter().map(|l| color_wave(l, i as usize)).collect(),
        })
        .collect()
}

fn generate_rainbow_animation(lines: &[String], count: u64, delay: u64) -> Vec<Frame> {
    (0..count)
        .map(|i| Frame {
            delay_ms: delay,
            lines: lines
                .iter()
                .map(|l| color_rainbow(l, i as usize, count as usize))
                .collect(),
        })
        .collect()
}

fn generate_sparkle_animation(lines: &[String], count: u64, delay: u64) -> Vec<Frame> {
    let mut rng: u32 = 1;
    (0..count)
        .map(|_| {
            rng = rng.wrapping_mul(1_103_515_245).wrapping_add(12_345);
            Frame {
                delay_ms: delay,
                lines: lines.iter().map(|l| color_sparkle(l, rng)).collect(),
            }
        })
        .collect()
}

fn generate_breathing_animation(lines: &[String], count: u64, delay: u64) -> Vec<Frame> {
    (0..count)
        .map(|i| Frame {
            delay_ms: delay,
            lines: lines
                .iter()
                .map(|l| color_breathing(l, i as usize, count as usize))
                .collect(),
        })
        .collect()
}

fn generate_static_animation(lines: &[String], _count: u64, delay: u64) -> Vec<Frame> {
    vec![Frame {
        delay_ms: delay,
        lines: lines.to_vec(),
    }]
}

fn generate_ascii_frame_animation(frame_sets: &[Vec<String>], count: u64, delay: u64) -> Vec<Frame> {
    if frame_sets.is_empty() {
        return Vec::new();
    }
    let max_h = frame_sets.iter().map(|f| f.len()).max().unwrap_or(0);
    let normalized: Vec<Vec<String>> = frame_sets
        .iter()
        .map(|f| {
            let mut n = f.clone();
            while n.len() < max_h {
                n.push(String::new());
            }
            n
        })
        .collect();

    (0..count)
        .map(|i| {
            let idx = i as usize % normalized.len();
            Frame {
                delay_ms: delay,
                lines: normalized[idx].clone(),
            }
        })
        .collect()
}

fn color_sweep(line: &str, offset: usize) -> String {
    const PALETTE: &[u8] = &[31, 32, 33, 34, 35, 36];
    let mut out = String::new();
    for (idx, ch) in line.chars().enumerate() {
        if ch == ' ' {
            out.push(' ');
            continue;
        }
        let color = PALETTE[(idx + offset) % PALETTE.len()];
        out.push_str(&format!("\x1b[{}m{}\x1b[0m", color, ch));
    }
    out
}

fn color_wave(line: &str, frame: usize) -> String {
    const PALETTE: &[u8] = &[31, 32, 33, 34, 35, 36];
    let mut out = String::new();
    for (idx, ch) in line.chars().enumerate() {
        if ch == ' ' {
            out.push(' ');
            continue;
        }
        let wave = ((idx as f64 * 0.5 + frame as f64 * 0.3).sin() * 3.0) as isize;
        let color = PALETTE[((wave + 3).max(0).min(5) as usize) % PALETTE.len()];
        out.push_str(&format!("\x1b[{}m{}\x1b[0m", color, ch));
    }
    out
}

fn color_rainbow(line: &str, frame: usize, total: usize) -> String {
    let palette = [
        (196u8, 0u8, 0u8),
        (208, 128, 0),
        (220, 220, 0),
        (0, 200, 0),
        (0, 150, 200),
        (100, 50, 200),
    ];
    let line_len = line.chars().count();
    let total = total.max(1);
    let mut out = String::new();
    for (idx, ch) in line.chars().enumerate() {
        if ch == ' ' {
            out.push(' ');
            continue;
        }
        let t = (idx as f64 / line_len.max(1) as f64 + frame as f64 / total as f64) % 1.0;
        let pi = t * (palette.len() - 1) as f64;
        let i0 = pi.floor() as usize;
        let i1 = (i0 + 1).min(palette.len() - 1);
        let frac = pi - pi.floor();
        let (r1, g1, b1) = palette[i0];
        let (r2, g2, b2) = palette[i1];
        let r = (r1 as f64 * (1.0 - frac) + r2 as f64 * frac) as u8;
        let g = (g1 as f64 * (1.0 - frac) + g2 as f64 * frac) as u8;
        let b = (b1 as f64 * (1.0 - frac) + b2 as f64 * frac) as u8;
        out.push_str(&format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, ch));
    }
    out
}

fn color_sparkle(line: &str, seed: u32) -> String {
    const PALETTE: &[u8] = &[31, 32, 33, 34, 35, 36, 91, 92, 93, 94, 95, 96];
    let mut rng = seed;
    let mut out = String::new();
    for ch in line.chars() {
        if ch == ' ' {
            out.push(' ');
            continue;
        }
        rng = rng.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        let bright = (rng >> 16) & 0xFF;
        if bright > 200 {
            let color = PALETTE[(rng as usize) % PALETTE.len()];
            out.push_str(&format!("\x1b[{}m{}\x1b[0m", color, ch));
        } else {
            out.push(ch);
        }
    }
    out
}

fn color_breathing(line: &str, frame: usize, total: usize) -> String {
    let total = total.max(1);
    let phase = (frame as f64 / total as f64 * std::f64::consts::PI * 2.0).sin();
    let brightness = ((phase * 0.5 + 0.5) * 155.0 + 100.0) as u8;
    let mut out = String::new();
    for ch in line.chars() {
        if ch == ' ' {
            out.push(' ');
            continue;
        }
        out.push_str(&format!(
            "\x1b[38;2;{};{};{}m{}\x1b[0m",
            brightness,
            brightness / 2,
            brightness / 3,
            ch
        ));
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
