mod config;
mod info;
mod plugins;
mod ui;

use crate::config::{generate_config, load_config};
use crate::info::Info;
use crate::plugins::{install_plugin, list_plugins};
use crate::ui::draw;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = None,
    after_help = "Examples:\n  xfetch\n  xfetch --config ~/.config/xfetch/config.jsonc\n  xfetch --gen-config\n  xfetch plugin install ./plugins/animate-logo\n  xfetch plugin list"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path to config file
    #[arg(short, long, global = true)]
    config: Option<String>,

    /// Generate a default config.jsonc (pacman layout) and exit
    #[arg(long, global = true)]
    gen_config: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Manage plugins
    Plugin {
        #[command(subcommand)]
        action: PluginCommands,
    },
}

#[derive(Subcommand, Debug)]
enum PluginCommands {
    /// Build and install a plugin
    ///
    /// Provide a local path (e.g. ./plugins/animate-logo) or just a plugin name.
    /// If the plugin is not found locally, it will be fetched from a remote repository.
    Install {
        /// Name or local path of the plugin (e.g., animate-logo or ./plugins/animate-logo)
        path: String,

        /// Git repository URL to fetch the plugin from
        ///
        /// Defaults to https://github.com/xscriptor/xfetch.git
        #[arg(long, short)]
        repo: Option<String>,
    },
    /// List installed plugins
    List,
}

fn main() {
    let cli = Cli::parse();

    if cli.gen_config {
        match generate_config(cli.config.clone()) {
            Ok(path) => {
                println!("Generated config: {}", path.display());
                println!("Run xfetch to see the new layout.");
                return;
            }
            Err(err) => {
                eprintln!("Failed to generate config: {}", err);
                std::process::exit(1);
            }
        }
    }

    match cli.command {
        Some(Commands::Plugin { action }) => match action {
            PluginCommands::Install { path, repo } => {
                match install_plugin(&path, repo.as_deref()) {
                    Ok(()) => {}
                    Err(err) => {
                        eprintln!("Error: {}", err);
                        std::process::exit(1);
                    }
                }
            }
            PluginCommands::List => {
                match list_plugins() {
                    Ok(plugins) => {
                        if plugins.is_empty() {
                            println!("No plugins installed.");
                            println!("Plugin directory: {}", plugins::default_plugin_dir().display());
                        } else {
                            println!("Installed plugins:");
                            for (name, path) in &plugins {
                                println!("  {}  ({})", name, path.display());
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("Error: {}", err);
                        std::process::exit(1);
                    }
                }
            }
        },
        None => {
            // Normal fetch behavior
            let config = load_config(cli.config);
            let info = Info::new();
            draw(&info, &config);
        }
    }
}
