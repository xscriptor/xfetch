use sysinfo::{Components, Disks, System};
use std::process::Command;

const UNKNOWN_GPU: &str = "Unknown GPU";
const ZERO_SWAP: &str = "0 B / 0 B (0%)";
const NA: &str = "N/A";
const BATT_DIR: &str = "/sys/class/power_supply";
const BATT_PREFIX: &str = "BAT";
const BATT_CAPACITY: &str = "capacity";
const BATT_STATUS: &str = "status";

const LSPCI_CMD: &str = "lspci";
const WMIC_CMD: &str = "wmic";
const SYSTEM_PROFILER_CMD: &str = "system_profiler";
const PMSET_CMD: &str = "pmset";

const GPU_CLASS_VGA: &str = "VGA";
const GPU_CLASS_3D: &str = "3D";
const GPU_CLASS_DISPLAY: &str = "Display";
const CHIPSET_MODEL: &str = "Chipset Model:";

pub fn get_cpu_info(sys: &System) -> String {
    let cpus = sys.cpus();
    if cpus.is_empty() {
        return super::unknown();
    }
    let brand = cpus[0].brand();
    let freq = cpus[0].frequency();
    let cores = cpus.len();
    format!("{} ({}) @ {:.2} GHz", brand, cores, freq as f64 / 1000.0)
}

pub fn get_gpu_info() -> Vec<String> {
    let mut gpus = Vec::new();
    if cfg!(target_os = "linux") {
        if let Ok(output) = Command::new(LSPCI_CMD).arg("-mm").output() {
            let out = String::from_utf8_lossy(&output.stdout);
            for line in out.lines() {
                if line.contains(GPU_CLASS_VGA) || line.contains(GPU_CLASS_3D) || line.contains(GPU_CLASS_DISPLAY) {
                    let parts: Vec<&str> = line.split('"').collect();
                    if parts.len() > 5 {
                        gpus.push(parts[5].to_string());
                    }
                }
            }
        }
    } else if cfg!(target_os = "windows") {
        if let Ok(output) = Command::new(WMIC_CMD)
            .args(["path", "win32_videocontroller", "get", "name"])
            .output()
        {
            let out = String::from_utf8_lossy(&output.stdout);
            for line in out.lines().skip(1) {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    gpus.push(trimmed.to_string());
                }
            }
        }
    } else if cfg!(target_os = "macos") {
        if let Ok(output) = Command::new(SYSTEM_PROFILER_CMD)
            .arg("SPDisplaysDataType")
            .output()
        {
            let out = String::from_utf8_lossy(&output.stdout);
            for line in out.lines() {
                if line.trim().starts_with(CHIPSET_MODEL) {
                    gpus.push(line.trim().replace(CHIPSET_MODEL, ""));
                }
            }
        }
    }
    if gpus.is_empty() {
        gpus.push(UNKNOWN_GPU.to_string());
    }
    gpus
}

pub fn get_memory_info(sys: &System) -> String {
    let total = super::b_to_gib(sys.total_memory());
    let used = super::b_to_gib(sys.used_memory());
    let percent = (used / total) * 100.0;
    format!("{:.2} GiB / {:.2} GiB ({:.0}%)", used, total, percent)
}

pub fn get_swap_info(sys: &System) -> String {
    let total = super::b_to_gib(sys.total_swap());
    let used = super::b_to_gib(sys.used_swap());
    if total == 0.0 {
        return ZERO_SWAP.to_string();
    }
    let percent = (used / total) * 100.0;
    format!("{:.2} GiB / {:.2} GiB ({:.0}%)", used, total, percent)
}

pub fn get_disk_info(disks: &Disks) -> Vec<String> {
    let mut disk_list = Vec::new();
    for disk in disks {
        let total = super::b_to_gib(disk.total_space());
        let available = super::b_to_gib(disk.available_space());
        let used = total - available;
        let percent = (used / total) * 100.0;
        let fs = disk
            .file_system()
            .to_str()
            .map(|s| s.to_string())
            .unwrap_or_else(super::unknown);
        disk_list.push(format!(
            "{:.2} GiB / {:.2} GiB ({:.0}%) - {}",
            used, total, percent, fs
        ));
    }
    disk_list
}

fn get_battery_info_macos() -> Option<String> {
    let output = Command::new(PMSET_CMD).args(["-g", "batt"]).output().ok()?;
    let out = String::from_utf8_lossy(&output.stdout);
    let line = out.lines().nth(1)?;
    let pct = line
        .split_whitespace()
        .find(|p| p.contains('%'))?
        .trim_end_matches(';')
        .to_string();
    let status = if line.contains("discharging") {
        "Discharging"
    } else if line.contains("charging") {
        "Charging"
    } else if line.contains("charged") {
        "Charged"
    } else {
        "Unknown"
    };
    Some(format!("{} [{}]", pct, status))
}

fn get_battery_info_linux() -> Option<String> {
    let batt_dir = std::path::Path::new(BATT_DIR);
    let entries = std::fs::read_dir(batt_dir).ok()?;
    let mut total_pct = 0u32;
    let mut batt_count = 0u32;
    let mut statuses: Vec<String> = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if !name.starts_with(BATT_PREFIX) {
            continue;
        }
        let base = entry.path();
        if let Ok(cap) = std::fs::read_to_string(base.join(BATT_CAPACITY)) {
            if let Ok(pct) = cap.trim().parse::<u32>() {
                total_pct += pct;
                batt_count += 1;
            }
        }
        if let Ok(s) = std::fs::read_to_string(base.join(BATT_STATUS)) {
            let s = s.trim().to_string();
            if !statuses.contains(&s) {
                statuses.push(s);
            }
        }
    }
    if batt_count > 0 {
        let avg = total_pct / batt_count;
        let status = if statuses.is_empty() {
            super::unknown()
        } else {
            statuses.join("+")
        };
        return Some(format!("{}% [{}]", avg, status));
    }
    None
}

fn get_battery_info_windows() -> Option<String> {
    let output = Command::new(WMIC_CMD)
        .args([
            "path",
            "Win32_Battery",
            "Get",
            "EstimatedChargeRemaining,BatteryStatus",
        ])
        .output()
        .ok()?;
    let out = String::from_utf8_lossy(&output.stdout);
    for line in out.lines().skip(1) {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let cols: Vec<&str> = trimmed.split_whitespace().collect();
        if cols.len() >= 2 {
            if let Ok(pct) = cols[0].parse::<u32>() {
                let status = match cols.get(1).and_then(|s| s.parse::<u32>().ok()) {
                    Some(1) => "Discharging",
                    Some(2) | Some(3) => "Charged",
                    Some(6) | Some(7) | Some(8) | Some(9) => "Charging",
                    _ => "Unknown",
                };
                return Some(format!("{}% [{}]", pct, status));
            }
        }
    }
    None
}

pub fn get_battery_info(_components: &Components) -> String {
    if cfg!(target_os = "macos") {
        if let Some(info) = get_battery_info_macos() {
            return info;
        }
    }
    if cfg!(target_os = "linux") {
        if let Some(info) = get_battery_info_linux() {
            return info;
        }
    }
    if cfg!(target_os = "windows") {
        if let Some(info) = get_battery_info_windows() {
            return info;
        }
    }
    NA.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_memory_info() {
        let mem = get_memory_info(&System::new_all());
        assert!(mem.contains("GiB"), "memory should show GiB");
    }

    #[test]
    fn test_get_swap_info() {
        let swap = get_swap_info(&System::new_all());
        assert!(swap.contains("GiB") || swap == ZERO_SWAP);
    }
}
