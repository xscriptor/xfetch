use crate::config::{Config};
use crate::info::Info;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::execute;
use std::io::stdout;
mod nodes;
use nodes::{prepare_render_tree};
mod renders;
use renders::{render_classic, render_classic_variants, render_side_block, render_tree, render_section};
mod x;
mod logo;






pub fn draw(info: &Info, config: &Config) {
    let mut stdout = stdout();

    // Prepare Render Tree
    let nodes = prepare_render_tree(info, &config.modules, config);

    // Get Logo Data (ASCII or Image)
    let (ascii_lines, image_printed, ascii_width) = logo::get_logo_data(config);
    

    // Render content to lines based on layout
    let layout_type = config.layout.as_deref().unwrap_or("default");
    let content_lines = match layout_type {
        "side-block" => render_side_block(&nodes, config),
        "tree" => render_tree(&nodes, config), // Image 2 style
        "section" => render_section(&nodes, config), // Image 3/4 style
        "pacman" | "box" | "line" | "dots" | "bottom_line" => render_classic_variants(&nodes, config, layout_type),
        _ => render_classic(&nodes, config),
    };

    let max_lines = std::cmp::max(ascii_lines.len(), content_lines.len());
    let gap = "  ";

    for i in 0..max_lines {
        // 1. Print Logo Part
        if image_printed {
            execute!(stdout, crossterm::cursor::MoveRight(ascii_width as u16)).unwrap();
            execute!(stdout, Print(gap)).unwrap();
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
                 execute!(stdout, Print(format!("{}{}", ascii_line, " ".repeat(padding)))).unwrap();
            } else {
                 // Calculate padding needed
                 let visible_len = console::measure_text_width(ascii_line);
                 let padding = if ascii_width > visible_len { ascii_width - visible_len } else { 0 };
                 
                 execute!(
                    stdout,
                    SetForegroundColor(Color::Rgb { r: 128, g: 128, b: 128 }),
                    Print(format!("{}{}", ascii_line, " ".repeat(padding))),
                    ResetColor
                ).unwrap();
            }
            execute!(stdout, Print(gap)).unwrap();
        }

        // 2. Print Info Part
        if i < content_lines.len() {
            execute!(stdout, Print(&content_lines[i])).unwrap();
        }
        execute!(stdout, Print("\n")).unwrap();
    }
}
