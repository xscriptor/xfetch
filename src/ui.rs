use crate::config::{Config};
use crate::info::Info;
use std::io::stdout;
mod nodes;
use nodes::{prepare_render_tree};
mod renders;
mod x;
mod logo;
mod layout;
mod print;






pub fn draw(info: &Info, config: &Config) {
    let _stdout = stdout();

    //1. Prepare Render Tree
    let nodes = prepare_render_tree(info, &config.modules, config);

    //2. Get Logo Data (ASCII or Image)
    let (ascii_lines, image_printed, ascii_width) = logo::get_logo_data(config);
    

    // 3. Get Layout Lines
    let content_lines = layout::get_content_lines(&nodes, config);

    // 4. Print everything
    print::print_output(ascii_lines, image_printed, ascii_width, content_lines, config);
}
