use crate::config::Config;
use super::nodes::RenderNode;
use super::renders::{render_classic, render_classic_variants, render_side_block, render_tree, render_section};

pub fn get_content_lines(nodes: &[RenderNode], config: &Config) -> Vec<String> {
    let layout_type = config.layout.as_deref().unwrap_or("default");
    match layout_type {
        "side-block" => render_side_block(nodes, config),
        "tree" => render_tree(nodes, config),
        "section" => render_section(nodes, config),
        "pacman" | "box" | "line" | "dots" | "bottom_line" => render_classic_variants(nodes, config, layout_type),
        _ => render_classic(nodes, config),
    }
}