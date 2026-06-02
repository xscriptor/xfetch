use serde::Deserialize;
use serde::Serialize;
use std::io::{self, Read, Write};
use std::process::Command;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct PluginRequest {
    version: Option<u32>,
    kind: Option<String>,
    args: Option<PluginArgs>,
}

#[derive(Debug, Deserialize)]
struct PluginArgs {
    username: Option<String>,
    token: Option<String>,
}

#[derive(Debug, Serialize)]
struct PluginResponse {
    lines: Vec<String>,
}

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let request: PluginRequest = match serde_json::from_str(&input) {
        Ok(value) => value,
        Err(_) => return,
    };

    let username = request
        .args
        .as_ref()
        .and_then(|a| a.username.clone())
        .or_else(|| std::env::var("GITHUB_USER").ok())
        .filter(|u| !u.is_empty());

    let lines = match username {
        Some(user) => get_github_stats(&user, request.args.as_ref().and_then(|a| a.token.as_deref())),
        None => vec![" GitHub: no username configured".to_string()],
    };

    let response = PluginResponse { lines };
    if let Ok(body) = serde_json::to_string(&response) {
        let _ = io::stdout().write_all(body.as_bytes());
    }
}

fn get_github_stats(username: &str, _token: Option<&str>) -> Vec<String> {
    let mut result = Vec::new();

    let api_url = format!("https://api.github.com/users/{}", username);
    let repos_url = format!("https://api.github.com/users/{}/repos?per_page=100&sort=pushed", username);

    let user_info = fetch_json(&api_url);
    let repos_info = fetch_json(&repos_url);

    match (user_info, repos_info) {
        (Ok(user), Ok(_repos)) => {
            let login = user["login"].as_str().unwrap_or(username);
            let name = user["name"].as_str().unwrap_or(login);
            let public_repos = user["public_repos"].as_u64().unwrap_or(0);
            let followers = user["followers"].as_u64().unwrap_or(0);
            let following = user["following"].as_u64().unwrap_or(0);
    result.push(format!(" {} (@{})", name, login));
            result.push(format!("   {} repos", public_repos));
            result.push(format!("   {} followers", followers));
            result.push(format!("   {} following", following));
        }
        (Ok(user), Err(_)) => {
            let login = user["login"].as_str().unwrap_or(username);
            let name = user["name"].as_str().unwrap_or(login);
            let public_repos = user["public_repos"].as_u64().unwrap_or(0);
            let followers = user["followers"].as_u64().unwrap_or(0);

            result.push(format!(" {} (@{})", name, login));
            result.push(format!("   {} repos", public_repos));
            result.push(format!("   {} followers", followers));
        }
        (Err(_), _) => {
            result.push(format!(" GitHub: could not fetch user '{}'", username));
        }
    }

    result
}

fn fetch_json(url: &str) -> Result<serde_json::Value, String> {
    let output = Command::new("curl")
        .args([
            "-s",
            "-H",
            "Accept: application/vnd.github.v3+json",
            "-H",
            "User-Agent: xfetch-plugin-github-stats/0.1",
            url,
        ])
        .output()
        .map_err(|e| format!("Failed to run curl: {}", e))?;

    if !output.status.success() {
        return Err("curl exited with error".to_string());
    }

    serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Failed to parse JSON: {}", e))
}
