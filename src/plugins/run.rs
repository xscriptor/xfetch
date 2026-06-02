use crate::config::{InfoPluginConfig, LogoAnimationConfig};
use crate::plugins::find_plugin_binary;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::Write;
use std::process::{Command, Stdio};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnimationFrame {
    pub delay_ms: u64,
    pub lines: Vec<String>,
}

#[derive(Debug, Serialize)]
struct InfoPluginRequest<'a> {
    version: u32,
    kind: &'a str,
    args: Option<&'a Value>,
}

#[derive(Debug, Deserialize)]
struct InfoPluginResponse {
    lines: Vec<String>,
}

fn run_plugin_raw(plugin_name: &str, payload: &[u8]) -> Result<Vec<u8>, String> {
    let plugin_path = find_plugin_binary(plugin_name)
        .ok_or_else(|| format!("Plugin not found: {}", plugin_name))?;

    let mut child = Command::new(plugin_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| format!("Failed to start plugin: {}", err))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(payload)
            .map_err(|err| format!("Failed to send plugin request: {}", err))?;
    }

    let output = child.wait_with_output()
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

    Ok(output.stdout)
}

pub fn run_logo_animation_plugin(
    plugin_name: &str,
    config: &LogoAnimationConfig,
    lines: &[String],
    frames: Option<Vec<Vec<String>>>,
) -> Result<Vec<AnimationFrame>, String> {
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

    let stdout = run_plugin_raw(plugin_name, &payload)?;

    let response: LogoAnimationResponse = serde_json::from_slice(&stdout)
        .map_err(|err| format!("Failed to parse plugin output: {}", err))?;

    if response.frames.is_empty() {
        return Err("Plugin returned no frames".to_string());
    }

    Ok(response.frames)
}

pub fn run_info_plugin(config: &InfoPluginConfig) -> Result<Vec<String>, String> {
    let request = InfoPluginRequest {
        version: 1,
        kind: "info_provider",
        args: config.args.as_ref(),
    };

    let payload = serde_json::to_vec(&request)
        .map_err(|err| format!("Failed to serialize plugin request: {}", err))?;

    let stdout = run_plugin_raw(&config.plugin, &payload)?;

    let response: InfoPluginResponse = serde_json::from_slice(&stdout)
        .map_err(|err| format!("Failed to parse plugin output: {}", err))?;

    Ok(response.lines)
}
