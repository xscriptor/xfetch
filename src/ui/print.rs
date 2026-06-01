use crate::config::Config;
use crate::plugins::AnimationFrame;
use crossterm::cursor::{Hide, MoveUp, Show};
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::execute;
use std::io::{stdout, Stdout};
use std::time::{Duration, Instant};

pub fn print_output(
    ascii_lines: Vec<String>,
    image_printed: bool,
    ascii_width: usize,
    content_lines: Vec<String>,
    config: &Config,
    force_plain_logo: bool,
) {
    let mut out = stdout();

    // Determine max lines to print (logo vs content)
    let max_lines = std::cmp::max(ascii_lines.len(), content_lines.len());
    let gap = "  ";

    for i in 0..max_lines {
        // 1. Print Logo Part
        if image_printed {
            execute!(out, crossterm::cursor::MoveRight(ascii_width as u16)).unwrap();
            execute!(out, Print(gap)).unwrap();
        } else {
            let ascii_line = if i < ascii_lines.len() {
                &ascii_lines[i]
            } else {
                ""
            };
            print_logo_line(&mut out, ascii_line, ascii_width, config, force_plain_logo);
            execute!(out, Print(gap)).unwrap();
        }

        // 2. Print Info Part
        if i < content_lines.len() {
            execute!(out, Print(&content_lines[i])).unwrap();
        }
        execute!(out, Print("\n")).unwrap();
    }
}

pub fn print_animated_output(
    frames: &[AnimationFrame],
    ascii_width: usize,
    content_lines: &[String],
    config: &Config,
    force_plain_logo: bool,
    duration_ms: Option<u64>,
    loop_enabled: bool,
) {
    if frames.is_empty() {
        return;
    }

    let mut out = stdout();
    let gap = "  ";
    let max_logo_width = max_frame_width(frames, ascii_width);
    let max_logo_lines = frames
        .iter()
        .map(|frame| frame.lines.len())
        .max()
        .unwrap_or(0);
    let max_lines = std::cmp::max(max_logo_lines, content_lines.len());
    let start = Instant::now();
    let duration_limit = duration_ms.map(Duration::from_millis);
    let loop_enabled = loop_enabled && duration_limit.is_some();
    let mut frame_index = 0;
    let mut first_frame = true;

    let _ = execute!(out, Hide);

    loop {
        let frame = &frames[frame_index];

        if !first_frame {
            let _ = execute!(out, MoveUp(max_lines as u16));
        }

        for i in 0..max_lines {
            let _ = execute!(out, Clear(ClearType::CurrentLine));
            let ascii_line = frame.lines.get(i).map(|line| line.as_str()).unwrap_or("");
            print_logo_line(&mut out, ascii_line, max_logo_width, config, force_plain_logo);
            let _ = execute!(out, Print(gap));
            if i < content_lines.len() {
                let _ = execute!(out, Print(&content_lines[i]));
            }
            let _ = execute!(out, Print("\n"));
        }

        let delay = std::cmp::max(1, frame.delay_ms);
        std::thread::sleep(Duration::from_millis(delay));

        if !loop_enabled {
            if frame_index + 1 >= frames.len() {
                break;
            }
        } else if let Some(limit) = duration_limit {
            if start.elapsed() >= limit {
                break;
            }
        }

        frame_index = (frame_index + 1) % frames.len();
        first_frame = false;
    }

    let _ = execute!(out, Show);
}

fn print_logo_line(
    out: &mut Stdout,
    ascii_line: &str,
    ascii_width: usize,
    config: &Config,
    force_plain_logo: bool,
) {
    let is_custom_ascii = force_plain_logo || config.ascii.is_some() || config.logo_path.is_some();
    let visible_len = visible_width(ascii_line);
    let padding = if ascii_width > visible_len {
        ascii_width - visible_len
    } else {
        0
    };

    if is_custom_ascii {
        execute!(out, Print(format!("{}{}", ascii_line, " ".repeat(padding)))).unwrap();
    } else {
        execute!(
            out,
            SetForegroundColor(Color::Rgb { r: 128, g: 128, b: 128 }),
            Print(format!("{}{}", ascii_line, " ".repeat(padding))),
            ResetColor
        )
        .unwrap();
    }
}

fn max_frame_width(frames: &[AnimationFrame], fallback: usize) -> usize {
    let mut max_width = fallback;
    for frame in frames {
        for line in &frame.lines {
            let width = visible_width(line);
            if width > max_width {
                max_width = width;
            }
        }
    }
    max_width
}

fn visible_width(value: &str) -> usize {
    let stripped = console::strip_ansi_codes(value);
    console::measure_text_width(&stripped)
}