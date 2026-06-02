use crate::plugins::{
    default_plugin_dir, plugin_binary_name, CARGO_CMD, CARGO_TOML, GIT_CMD, PLUGIN_PREFIX,
    PLUGINS_DIR, TARGET_RELEASE,
};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn has_path_separator(name: &str) -> bool {
    name.contains('/') || name.contains('\\')
}

fn resolve_local_plugin_dir(path: &str) -> Result<PathBuf, String> {
    let candidate = PathBuf::from(path);

    if candidate.is_dir() {
        return Ok(candidate);
    }

    if let Ok(cwd) = env::current_dir() {
        let in_plugins = cwd.join(PLUGINS_DIR).join(path);
        if in_plugins.is_dir() {
            return Ok(in_plugins);
        }
    }

    Err(format!("Plugin not found locally: '{}'", path))
}

pub fn install_plugin(name_or_path: &str, repo: Option<&str>) -> Result<(), String> {
    let plugin_dir = resolve_local_plugin_dir(name_or_path);

    match plugin_dir {
        Ok(dir) => build_and_install_plugin(&dir, name_or_path),
        Err(_) if repo.is_some() || has_path_separator(name_or_path) => {
            let repo_url = repo.unwrap_or(super::DEFAULT_PLUGIN_REPO);
            install_remote_plugin(name_or_path, repo_url)
        }
        Err(_) => install_remote_plugin(name_or_path, super::DEFAULT_PLUGIN_REPO),
    }
}

fn build_and_install_plugin(plugin_dir: &Path, name: &str) -> Result<(), String> {
    if !plugin_dir.join(CARGO_TOML).exists() {
        let display = plugin_dir.display();
        if name.contains('/') || name.contains('\\') {
            return Err(format!("No Cargo.toml found in '{}'", display));
        }
        return Err(format!(
            "Plugin '{}' not found locally and could not be fetched remotely.\n\
             No Cargo.toml found in '{}'.\n\
             Try specifying a different path or check the plugin name.",
            name, display
        ));
    }

    let plugin_name = plugin_dir
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| "Invalid plugin directory name".to_string())?;

    println!("Building plugin '{}'...", plugin_name);
    let status = Command::new(CARGO_CMD)
        .args(["build", "--release"])
        .current_dir(plugin_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|err| format!("Failed to run cargo: {}", err))?;

    if !status.success() {
        return Err("Cargo build failed".to_string());
    }

    let binary_name = plugin_binary_name(plugin_name);
    let built_binary = plugin_dir
        .join(TARGET_RELEASE)
        .join(&binary_name);

    if !built_binary.is_file() {
        return Err(format!(
            "Built binary not found at '{}'",
            built_binary.display()
        ));
    }

    let dest_dir = default_plugin_dir();
    fs::create_dir_all(&dest_dir)
        .map_err(|err| format!("Failed to create plugin directory: {}", err))?;

    let dest_path = dest_dir.join(&binary_name);
    fs::copy(&built_binary, &dest_path)
        .map_err(|err| format!("Failed to copy plugin binary: {}", err))?;

    println!("Installed plugin '{}' to {}", plugin_name, dest_path.display());
    Ok(())
}

fn install_remote_plugin(name: &str, repo_url: &str) -> Result<(), String> {
    let temp_dir = env::temp_dir().join(format!("{}{}", PLUGIN_PREFIX, name));
    let plugin_path = temp_dir.join(PLUGINS_DIR).join(name);

    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)
            .map_err(|err| format!("Failed to clean temp directory: {}", err))?;
    }

    let repo_display = repo_url.trim_end_matches(".git");
    println!("Fetching plugin '{}' from {}...", name, repo_display);

    let status = Command::new(GIT_CMD)
        .args(["clone", "--depth", "1", repo_url])
        .arg(&temp_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|err| format!("Failed to run git: {}. Is git installed?", err))?;

    if !status.success() {
        let _ = fs::remove_dir_all(&temp_dir);
        return Err("Failed to clone repository".to_string());
    }

    if !plugin_path.is_dir() {
        let _ = fs::remove_dir_all(&temp_dir);
        return Err(format!(
            "Plugin '{}' not found in repository '{}'.\n\
             Available plugins can be found at {}/tree/main/plugins",
            name, repo_display, repo_display
        ));
    }

    let result = build_and_install_plugin(&plugin_path, name);

    let _ = fs::remove_dir_all(&temp_dir);
    result
}
