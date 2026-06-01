use crate::config::LogoAnimationConfig;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::io::Write;

const DEFAULT_PLUGIN_REPO: &str = "https://github.com/xscriptor/xfetch.git";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnimationFrame {
    pub delay_ms: u64,
    pub lines: Vec<String>,
}

#[derive(Debug, Serialize)]
struct LogoAnimationRequest<'a> {
    version: u32,
    kind: &'a str,
    lines: &'a [String],
    frames: Option<Vec<Vec<String>>>,
    args: LogoAnimationArgs,
}

#[derive(Debug, Serialize)]
struct LogoAnimationArgs {
    fps: Option<u64>,
    duration_ms: Option<u64>,
    #[serde(rename = "loop")]
    loop_enabled: Option<bool>,
    style: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LogoAnimationResponse {
    frames: Vec<AnimationFrame>,
}

pub fn run_logo_animation_plugin(
    plugin_name: &str,
    config: &LogoAnimationConfig,
    lines: &[String],
    frames: Option<Vec<Vec<String>>>,
) -> Result<Vec<AnimationFrame>, String> {
    let plugin_path = find_plugin_binary(plugin_name)
        .ok_or_else(|| format!("Plugin not found: {}", plugin_name))?;

    let request = LogoAnimationRequest {
        version: 1,
        kind: "logo_animation",
        lines,
        frames,
        args: LogoAnimationArgs {
            fps: config.fps,
            duration_ms: config.duration_ms,
            loop_enabled: config.loop_enabled,
            style: config.style.clone(),
        },
    };

    let payload = serde_json::to_vec(&request)
        .map_err(|err| format!("Failed to serialize plugin request: {}", err))?;

    let mut child = Command::new(plugin_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| format!("Failed to start plugin: {}", err))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(&payload)
            .map_err(|err| format!("Failed to send plugin request: {}", err))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|err| format!("Failed to read plugin output: {}", err))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let msg = if stderr.trim().is_empty() {
            "Plugin exited with error".to_string()
        } else {
            stderr.trim().to_string()
        };
        return Err(msg);
    }

    let response: LogoAnimationResponse = serde_json::from_slice(&output.stdout)
        .map_err(|err| format!("Failed to parse plugin output: {}", err))?;

    if response.frames.is_empty() {
        return Err("Plugin returned no frames".to_string());
    }

    Ok(response.frames)
}

pub fn install_plugin(name_or_path: &str, repo: Option<&str>) -> Result<(), String> {
    let plugin_dir = resolve_local_plugin_dir(name_or_path);

    match plugin_dir {
        Ok(dir) => build_and_install_plugin(&dir, name_or_path),
        Err(_) if repo.is_some() || has_path_separator(name_or_path) => {
            let repo_url = repo.unwrap_or(DEFAULT_PLUGIN_REPO);
            install_remote_plugin(name_or_path, repo_url)
        }
        Err(_) => install_remote_plugin(name_or_path, DEFAULT_PLUGIN_REPO),
    }
}

fn has_path_separator(name: &str) -> bool {
    name.contains('/') || name.contains('\\')
}

fn build_and_install_plugin(plugin_dir: &Path, name: &str) -> Result<(), String> {
    if !plugin_dir.join("Cargo.toml").exists() {
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
    let status = Command::new("cargo")
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
        .join("target")
        .join("release")
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
    let temp_dir = env::temp_dir().join(format!("xfetch-plugin-{}", name));
    let plugin_path = temp_dir.join("plugins").join(name);

    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)
            .map_err(|err| format!("Failed to clean temp directory: {}", err))?;
    }

    let repo_display = repo_url.trim_end_matches(".git");
    println!("Fetching plugin '{}' from {}...", name, repo_display);

    let status = Command::new("git")
        .args([
            "clone",
            "--depth", "1",
            repo_url,
        ])
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

pub fn remove_plugin(name: &str) -> Result<(), String> {
    let binary_name = plugin_binary_name(name);
    let plugin_dir = default_plugin_dir();
    let binary_path = plugin_dir.join(&binary_name);

    if binary_path.is_file() {
        fs::remove_file(&binary_path)
            .map_err(|err| format!("Failed to remove plugin '{}': {}", name, err))?;
        println!("Removed plugin '{}'", name);
        Ok(())
    } else {
        Err(format!(
            "Plugin '{}' is not installed (not found at {})",
            name,
            binary_path.display()
        ))
    }
}

pub fn list_plugins() -> Result<Vec<(String, PathBuf)>, String> {
    let mut plugins = Vec::new();

    let plugin_dir = default_plugin_dir();
    if plugin_dir.is_dir() {
        for entry in fs::read_dir(&plugin_dir).map_err(|err| format!("Failed to read plugin directory: {}", err))? {
            let entry = entry.map_err(|err| format!("Failed to read entry: {}", err))?;
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = extract_plugin_name(&path) {
                    plugins.push((name, path));
                }
            }
        }
    }

    plugins.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(plugins)
}

fn extract_plugin_name(path: &Path) -> Option<String> {
    let filename = path.file_name()?.to_str()?;
    let prefix = if cfg!(target_os = "windows") {
        "xfetch-plugin-"
    } else {
        "xfetch-plugin-"
    };

    if let Some(name) = filename.strip_prefix(prefix) {
        if cfg!(target_os = "windows") {
            name.strip_suffix(".exe").map(|n| n.to_string())
        } else {
            Some(name.to_string())
        }
    } else {
        None
    }
}

fn resolve_local_plugin_dir(path: &str) -> Result<PathBuf, String> {
    let candidate = PathBuf::from(path);

    if candidate.is_dir() {
        return Ok(candidate);
    }

    if let Ok(cwd) = env::current_dir() {
        let in_plugins = cwd.join("plugins").join(path);
        if in_plugins.is_dir() {
            return Ok(in_plugins);
        }
    }

    Err(format!("Plugin not found locally: '{}'", path))
}

fn find_plugin_binary(plugin_name: &str) -> Option<PathBuf> {
    let direct_path = Path::new(plugin_name);
    if direct_path.components().count() > 1 && direct_path.is_file() {
        return Some(direct_path.to_path_buf());
    }

    let binary_name = plugin_binary_name(plugin_name);

    if let Some(path) = find_in_path(&binary_name) {
        return Some(path);
    }

    let config_dir = default_plugin_dir();
    let in_config_dir = config_dir.join(&binary_name);
    if in_config_dir.is_file() {
        return Some(in_config_dir);
    }

    if let Ok(cwd) = env::current_dir() {
        let dev_path = cwd
            .join("plugins")
            .join(plugin_name)
            .join("target")
            .join("release")
            .join(&binary_name);
        if dev_path.is_file() {
            return Some(dev_path);
        }

        let dev_path_flat = cwd
            .join("plugins")
            .join(plugin_name)
            .join(&binary_name);
        if dev_path_flat.is_file() {
            return Some(dev_path_flat);
        }
    }

    None
}

fn find_in_path(binary_name: &str) -> Option<PathBuf> {
    let path_var = env::var_os("PATH")?;
    for dir in env::split_paths(&path_var) {
        let candidate = dir.join(binary_name);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}

pub fn default_plugin_dir() -> PathBuf {
    let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_dir.join("xfetch").join("plugins")
}

fn plugin_binary_name(plugin_name: &str) -> String {
    if cfg!(target_os = "windows") {
        format!("xfetch-plugin-{}.exe", plugin_name)
    } else {
        format!("xfetch-plugin-{}", plugin_name)
    }
}
