use crate::config::Config;
use viuer::{print_from_file, Config as ViuerConfig};
use crossterm::execute;
use std::io::stdout;
use super::x::{expand_path, get_default_ascii};


pub fn get_logo_data(config: &Config) -> (Vec<String>, bool, usize) {
// ASCII/Image handling
    let mut ascii_lines: Vec<String> = Vec::new();
    let mut image_printed = false;
    let mut ascii_width = 0;
    let mut stdout = stdout();

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
    (ascii_lines, image_printed, ascii_width)
}


//tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_get_logo_data_default() {
        // Verifica que el comportamiento por defecto devuelva ASCII y no marque imagen impresa
        let config = Config::default();
        let (ascii_lines, is_image, _width) = get_logo_data(&config);
        
        assert!(!is_image);
        assert!(!ascii_lines.is_empty());
    }
}