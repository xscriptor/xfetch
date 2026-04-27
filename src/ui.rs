use crate::config::{Config};
use crate::info::Info;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::execute;
use std::io::stdout;
use viuer::{print_from_file, Config as ViuerConfig};
use std::path::PathBuf;
mod nodes;
use nodes::{prepare_render_tree};
mod renders;
use renders::{render_classic, render_classic_variants, render_side_block, render_tree, render_section};


fn expand_path(path: &str) -> PathBuf {
    if path.starts_with("~") {
        if let Some(home) = dirs::home_dir() {
            return home.join(&path[2..]);
        }
    }
    PathBuf::from(path)
}



pub fn draw(info: &Info, config: &Config) {
    let mut stdout = stdout();

    // Prepare Render Tree
    let nodes = prepare_render_tree(info, &config.modules, config);

    // ASCII/Image handling
    let mut ascii_lines: Vec<String> = Vec::new();
    let mut image_printed = false;
    let mut ascii_width = 0;

    if let Some(path_str) = &config.logo_path {
        let path = expand_path(path_str);
        if path_str.ends_with(".png") || path_str.ends_with(".jpg") || path_str.ends_with(".jpeg") || path_str.ends_with(".svg") {
            let conf = ViuerConfig {
                absolute_offset: false,
                transparent: true,
                ..Default::default()
            };
            if let Ok((width, height)) = print_from_file(&path, &conf) {
                image_printed = true;
                ascii_width = width as usize;
                execute!(stdout, crossterm::cursor::MoveUp(height as u16)).unwrap();
            }
        } else {
             if let Ok(content) = std::fs::read_to_string(&path) {
                 for line in content.lines() {
                     ascii_lines.push(line.to_string());
                 }
             }
        }
    } else if let Some(path_str) = &config.ascii {
        let path = expand_path(path_str);
        if let Ok(content) = std::fs::read_to_string(&path) {
             for line in content.lines() {
                 ascii_lines.push(line.to_string());
             }
        }
    } else {
        let default_art = get_default_ascii();
        for line in default_art.lines() {
            ascii_lines.push(line.to_string());
        }
    }

    if !image_printed && !ascii_lines.is_empty() {
        // Trim trailing spaces from ascii lines to avoid excessive width
        ascii_lines = ascii_lines.into_iter().map(|l| l.trim_end().to_string()).collect();
        // Use console::measure_text_width to get accurate display width (handling wide chars correctly)
        ascii_width = ascii_lines.iter().map(|l| console::measure_text_width(l)).max().unwrap_or(0);
    }

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


fn get_default_ascii() -> String {
    r#"
__  __
  \ \/ /
   \  /
   /  \
  /_/\_\
 /____/linux
---------BEGIN PUBLIC KEY----------
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMII
BCgKCAQEAwtU/XOS/xOf/FakeKeyDataFor
ArtPutHere+Of/XOSLINUXDISTRO/gD4t4+
N4ToR3aL/K3yG3NkR/v3RyL0ngD4t4+XoSy
LiNuX/R0cK/M0r3+L1nUx/D1sTr0/R4cK3a
XYS/aBcDeFgHiJkLmNoPqRsTuVwXyZ01234
56789+/woXosLinuxR0xOsD1sTr0d1stR0f
0/c00L/k3Y/wER3aL63nD/v3RyL0ngD4t4a
ABcD/EfGhIjK/xOsLiNuX/D1sTr0/R0cK3a
LmNoPqRsTuVwXyZ0/1nUx/wER3aL63nD/v3
RyL0ngD4t4+XoSy/M0r3+L1nUx/D1sTr0/R
4cK3a/XYS/aBcDeFgHiJkLmNoPqRsTuVwXy
Z0123456789+/woXos/Linux/Rocks/==
----------END PUBLIC KEY-----------
"#.trim().to_string()
}
