use crate::config::LogoAnimationConfig;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::io::Write;

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
    args: LogoAnimationArgs,
}

#[derive(Debug, Serialize)]
struct LogoAnimationArgs {
    fps: Option<u64>,
    duration_ms: Option<u64>,
    #[serde(rename = "loop")]
    loop_enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct LogoAnimationResponse {
    frames: Vec<AnimationFrame>,
}

pub fn run_logo_animation_plugin(
    plugin_name: &str,
    config: &LogoAnimationConfig,
    lines: &[String],
) -> Result<Vec<AnimationFrame>, String> {
    let plugin_path = find_plugin_binary(plugin_name)
        .ok_or_else(|| format!("Plugin not found: {}", plugin_name))?;

    let request = LogoAnimationRequest {
        version: 1,
        kind: "logo_animation",
        lines,
        args: LogoAnimationArgs {
            fps: config.fps,
            duration_ms: config.duration_ms,
            loop_enabled: config.loop_enabled,
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

fn default_plugin_dir() -> PathBuf {
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
