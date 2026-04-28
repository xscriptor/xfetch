use crate::config::Config;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::execute;
use std::io::stdout; 

pub fn print_output(ascii_lines: Vec<String>, image_printed: bool, ascii_width: usize, content_lines: Vec<String>, config: &Config) {
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
            let is_custom_ascii = config.ascii.is_some() || config.logo_path.is_some();
            if is_custom_ascii {
                 // Calculate padding needed: width - visible_width(ascii_line)
                 let visible_len = console::measure_text_width(ascii_line);
                 let padding = if ascii_width > visible_len { ascii_width - visible_len } else { 0 };
                 execute!(out, Print(format!("{}{}", ascii_line, " ".repeat(padding)))).unwrap();
            } else {
                 // Calculate padding needed
                 let visible_len = console::measure_text_width(ascii_line);
                 let padding = if ascii_width > visible_len { ascii_width - visible_len } else { 0 };
                 
                 execute!(
                    out,
                    SetForegroundColor(Color::Rgb { r: 128, g: 128, b: 128 }),
                    Print(format!("{}{}", ascii_line, " ".repeat(padding))),
                    ResetColor
                ).unwrap();
            }
            execute!(out, Print(gap)).unwrap();
        }

        // 2. Print Info Part
        if i < content_lines.len() {
            execute!(out, Print(&content_lines[i])).unwrap();
        }
        execute!(out, Print("\n")).unwrap();
    }
}