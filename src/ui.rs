use crate::config::Config;
use crate::info::Info;
use crate::plugins::run_logo_animation_plugin;
use console::strip_ansi_codes;
use std::io::{stdout, IsTerminal};
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

    if !image_printed && !ascii_lines.is_empty() {
        if let Some(animation_config) = &config.logo_animation {
            if let Some(plugin_name) = animation_config.plugin.as_deref() {
                if stdout().is_terminal() {
                    if let Ok(mut frames) =
                        run_logo_animation_plugin(plugin_name, animation_config, &ascii_lines)
                    {
                        if !config.show_colors {
                            for frame in &mut frames {
                                frame.lines = frame
                                    .lines
                                    .iter()
                                    .map(|line| strip_ansi_codes(line).to_string())
                                    .collect();
                            }
                        }

                        print::print_animated_output(
                            &frames,
                            ascii_width,
                            &content_lines,
                            config,
                            true,
                            animation_config.duration_ms,
                            animation_config.loop_enabled.unwrap_or(false),
                        );
                        return;
                    }
                }
            }
        }
    }

    // 4. Print everything
    print::print_output(
        ascii_lines,
        image_printed,
        ascii_width,
        content_lines,
        config,
        false,
    );
}
