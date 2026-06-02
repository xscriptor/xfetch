use sysinfo::{
    Components, Disks, Networks, System,
};
use std::env;
#[cfg(target_os = "linux")]
use std::fs;
use std::process::Command;

const BYTES_PER_GIB: f64 = 1024.0 * 1024.0 * 1024.0;

fn unknown() -> String {
    "Unknown".to_string()
}

fn b_to_gib(bytes: u64) -> f64 {
    bytes as f64 / BYTES_PER_GIB
}

pub struct Info {
    pub os: String,
    pub kernel: String,
    pub host_name: String,
    pub shell: String,
    pub terminal: String,
    pub cpu: String,
    pub gpu: Vec<String>,
    pub memory: String,
    pub swap: String,
    pub disks: Vec<String>,
    pub battery: String,
    pub uptime: String,
    pub packages: String,
    pub desktop: String,
    pub user: String,
    pub datetime: String,
    pub local_ip: String,
}

impl Info {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        let disks = Disks::new_with_refreshed_list();
        let networks = Networks::new_with_refreshed_list();
        let components = Components::new_with_refreshed_list();

        Self {
            os: get_os_info(),
            kernel: get_kernel_info(),
            host_name: get_host_name(),
            shell: get_shell_info(),
            terminal: get_terminal_info(),
            cpu: get_cpu_info(&sys),
            gpu: get_gpu_info(),
            memory: get_memory_info(&sys),
            swap: get_swap_info(&sys),
            disks: get_disk_info(&disks),
            battery: get_battery_info(&components),
            uptime: get_uptime_info(),
            packages: get_packages_info(),
            desktop: get_desktop_info(),
            user: get_user_info(),
            datetime: get_datetime_info(),
            local_ip: get_local_ip_info(&networks),
        }
    }
}

fn get_os_info() -> String {
    let name = System::name().unwrap_or_else(unknown);
    let version = System::os_version().unwrap_or_default();
    let arch = std::env::consts::ARCH;
    format!("{} {} {}", name, version, arch)
}

fn get_kernel_info() -> String {
    System::kernel_version().unwrap_or_else(unknown)
}

fn get_host_name() -> String {
    System::host_name().unwrap_or_else(unknown)
}

fn get_shell_info() -> String {
    if let Ok(shell) = env::var("SHELL") {
        let path = std::path::Path::new(&shell);
        if let Some(name) = path.file_name() {
            return name.to_string_lossy().into_owned();
        }
    }
    if cfg!(target_os = "windows") {
        if env::var("PSModulePath").is_ok() {
            return "PowerShell".to_string();
        }
        return "cmd".to_string();
    }
    "Unknown".to_string()
}

fn get_terminal_info() -> String {
    if let Ok(term) = env::var("TERM_PROGRAM") {
        return term;
    }
    if let Ok(_) = env::var("WT_SESSION") {
        return "Windows Terminal".to_string();
    }
    if let Ok(term) = env::var("TERM") {
        return term;
    }
    "Unknown".to_string()
}

fn get_cpu_info(sys: &System) -> String {
    let cpus = sys.cpus();
    if cpus.is_empty() {
        return unknown();
    }
    let brand = cpus[0].brand();
    let freq = cpus[0].frequency();
    let cores = cpus.len();
    format!("{} ({}) @ {:.2} GHz", brand, cores, freq as f64 / 1000.0)
}

fn get_gpu_info() -> Vec<String> {
    let mut gpus = Vec::new();
    if cfg!(target_os = "linux") {
        if let Ok(output) = Command::new("lspci").arg("-mm").output() {
            let out = String::from_utf8_lossy(&output.stdout);
            for line in out.lines() {
                if line.contains("VGA") || line.contains("3D") || line.contains("Display") {
                     let parts: Vec<&str> = line.split('"').collect();
                     if parts.len() > 5 {
                         gpus.push(parts[5].to_string());
                     }
                }
            }
        }
    }
    else if cfg!(target_os = "windows") {
        if let Ok(output) = Command::new("wmic").args(&["path", "win32_videocontroller", "get", "name"]).output() {
            let out = String::from_utf8_lossy(&output.stdout);
            for line in out.lines().skip(1) {
                let trimmed = line.trim();
                if !trimmed.is_empty() {
                    gpus.push(trimmed.to_string());
                }
            }
        }
    }
    else if cfg!(target_os = "macos") {
        if let Ok(output) = Command::new("system_profiler").arg("SPDisplaysDataType").output() {
             let out = String::from_utf8_lossy(&output.stdout);
             for line in out.lines() {
                 if line.trim().starts_with("Chipset Model:") {
                     gpus.push(line.trim().replace("Chipset Model: ", ""));
                 }
             }
        }
    }
    if gpus.is_empty() {
        gpus.push("Unknown GPU".to_string());
    }
    gpus
}

fn get_memory_info(sys: &System) -> String {
    let total = b_to_gib(sys.total_memory());
    let used = b_to_gib(sys.used_memory());
    let percent = (used / total) * 100.0;
    format!("{:.2} GiB / {:.2} GiB ({:.0}%)", used, total, percent)
}

fn get_swap_info(sys: &System) -> String {
    let total = b_to_gib(sys.total_swap());
    let used = b_to_gib(sys.used_swap());
    if total == 0.0 {
        return "0 B / 0 B (0%)".to_string();
    }
    let percent = (used / total) * 100.0;
    format!("{:.2} GiB / {:.2} GiB ({:.0}%)", used, total, percent)
}

fn get_disk_info(disks: &Disks) -> Vec<String> {
    let mut disk_list = Vec::new();
    for disk in disks {
        let total = b_to_gib(disk.total_space());
        let available = b_to_gib(disk.available_space());
        let used = total - available;
        let percent = (used / total) * 100.0;
        let fs = disk.file_system().to_str().map(|s| s.to_string()).unwrap_or_else(unknown);
        disk_list.push(format!("{:.2} GiB / {:.2} GiB ({:.0}%) - {}", used, total, percent, fs));
    }
    disk_list
}

fn get_battery_info(_components: &Components) -> String {
    #[cfg(target_os = "linux")]
    {
        let batt_dir = std::path::Path::new("/sys/class/power_supply");
        if let Ok(entries) = fs::read_dir(batt_dir) {
            let mut total_pct = 0u32;
            let mut batt_count = 0u32;
            let mut statuses = Vec::new();
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                if !name.starts_with("BAT") {
                    continue;
                }
                let base = entry.path();
                if let Ok(cap) = fs::read_to_string(base.join("capacity")) {
                    if let Ok(pct) = cap.trim().parse::<u32>() {
                        total_pct += pct;
                        batt_count += 1;
                    }
                }
                if let Ok(s) = fs::read_to_string(base.join("status")) {
                    let s = s.trim().to_string();
                    if !statuses.contains(&s) {
                        statuses.push(s);
                    }
                }
            }
            if batt_count > 0 {
                let avg = total_pct / batt_count;
                let status = if statuses.is_empty() { unknown() }
                    else { statuses.join("+") };
                return format!("{}% [{}]", avg, status);
            }
        }
    }
    if cfg!(target_os = "macos") {
        if let Ok(output) = Command::new("pmset").args(["-g", "batt"]).output() {
            let out = String::from_utf8_lossy(&output.stdout);
            if let Some(line) = out.lines().nth(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(pct) = parts.iter().find(|p| p.ends_with('%')) {
                    let status = if line.contains("discharging") { "Discharging" }
                        else if line.contains("charging") { "Charging" }
                        else if line.contains("charged") { "Charged" }
                        else { "Unknown" };
                    return format!("{} [{}]", pct, status);
                }
            }
        }
    }
    if cfg!(target_os = "windows") {
        if let Ok(output) = Command::new("wmic")
            .args(["path", "Win32_Battery", "Get", "EstimatedChargeRemaining,BatteryStatus"])
            .output()
        {
            let out = String::from_utf8_lossy(&output.stdout);
            for line in out.lines().skip(1) {
                let trimmed = line.trim();
                if trimmed.is_empty() { continue; }
                let cols: Vec<&str> = trimmed.split_whitespace().collect();
                if cols.len() >= 2 {
                    if let Ok(pct) = cols[0].parse::<u32>() {
                        let status = match cols.get(1).and_then(|s| s.parse::<u32>().ok()) {
                            Some(1) => "Discharging",
                            Some(2) | Some(3) => "Charged",
                            Some(6) | Some(7) | Some(8) | Some(9) => "Charging",
                            _ => "Unknown",
                        };
                        return format!("{}% [{}]", pct, status);
                    }
                }
            }
        }
    }
    "N/A".to_string()
}

fn get_uptime_info() -> String {
    let uptime = System::uptime();
    let hours = uptime / 3600;
    let mins = (uptime % 3600) / 60;
    format!("{} hours, {} mins", hours, mins)
}

fn get_packages_info() -> String {
    if cfg!(target_os = "linux") {
        if let Ok(output) = Command::new("pacman").arg("-Qq").output() {
            if output.status.success() {
                 let count = String::from_utf8_lossy(&output.stdout).lines().count();
                 return format!("{} (pacman)", count);
            }
        }
        if let Ok(output) = Command::new("dpkg").arg("--get-selections").output() {
             if output.status.success() {
                 let count = String::from_utf8_lossy(&output.stdout).lines().count();
                 return format!("{} (dpkg)", count);
            }
        }
    }
    if cfg!(target_os = "windows") {
        if let Ok(output) = Command::new("scoop").arg("list").output() {
             if output.status.success() {
                 let count = String::from_utf8_lossy(&output.stdout).lines().count().saturating_sub(4);
                 return format!("{} (scoop)", count);
            }
        }
    }
    if cfg!(target_os = "macos") {
        if let Ok(output) = Command::new("brew").arg("list").arg("--formula").output() {
             if output.status.success() {
                 let count = String::from_utf8_lossy(&output.stdout).lines().count();
                 return format!("{} (brew)", count);
            }
        }
    }
    "Unknown".to_string()
}

fn get_desktop_info() -> String {
    if let Ok(de) = env::var("XDG_CURRENT_DESKTOP") {
        return de;
    }
    if let Ok(de) = env::var("DESKTOP_SESSION") {
        return de;
    }
    if cfg!(target_os = "windows") {
        return "Explorer".to_string();
    }
    if cfg!(target_os = "macos") {
        return "Aqua".to_string();
    }
    unknown()
}

fn get_user_info() -> String {
    env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .unwrap_or_else(|_| unknown())
}

fn get_datetime_info() -> String {
    if cfg!(target_os = "windows") {
        if let Ok(output) = Command::new("powershell").arg("-Command").arg("Get-Date -Format 'yyyy-MM-dd HH:mm:ss'").output() {
             return String::from_utf8_lossy(&output.stdout).trim().to_string();
        }
    } else {
        if let Ok(output) = Command::new("date").arg("+%Y-%m-%d %H:%M:%S").output() {
            return String::from_utf8_lossy(&output.stdout).trim().to_string();
        }
    }
    unknown()
}

fn get_local_ip_info(networks: &Networks) -> String {
    for (_name, data) in networks {
        for ip in data.ip_networks() {
             if let std::net::IpAddr::V4(ipv4) = ip.addr {
                 if !ipv4.is_loopback() {
                     return ipv4.to_string();
                 }
             }
        }
    }
    "127.0.0.1".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_info() {
        let user = get_user_info();
        assert!(!user.is_empty(), "user info should not be empty");
        // Should resolve to either USER or USERNAME
        let expected = env::var("USER")
            .or_else(|_| env::var("USERNAME"))
            .unwrap_or_else(|_| "Unknown".to_string());
        assert_eq!(user, expected);
    }

    #[test]
    fn test_get_uptime_info() {
        let uptime = get_uptime_info();
        assert!(uptime.contains("hours") || uptime.contains("mins"));
    }

    #[test]
    fn test_get_datetime_info() {
        let dt = get_datetime_info();
        assert!(dt.len() >= 10, "datetime should be at least YYYY-MM-DD: got '{}'", dt);
    }

    #[test]
    fn test_get_os_info() {
        let os = get_os_info();
        assert!(!os.is_empty(), "OS info should not be empty");
    }

    #[test]
    fn test_get_kernel_info() {
        let kernel = get_kernel_info();
        assert!(!kernel.is_empty(), "kernel info should not be empty");
    }

    #[test]
    fn test_get_host_name() {
        let host = get_host_name();
        assert!(!host.is_empty(), "host name should not be empty");
    }

    #[test]
    fn test_get_memory_info() {
        let mem = get_memory_info(&System::new_all());
        assert!(mem.contains("GiB"), "memory should show GiB");
    }

    #[test]
    fn test_get_swap_info() {
        let swap = get_swap_info(&System::new_all());
        assert!(swap.contains("GiB") || swap.contains("0 B"));
    }

    #[test]
    fn test_get_battery_info_fallback() {
        let battery = get_battery_info(&Components::new_with_refreshed_list());
        // Should not panic; format is either percentage or "N/A"
        assert!(battery.contains('%') || battery == "N/A");
    }

    #[test]
    fn test_get_gpu_not_empty() {
        let gpus = get_gpu_info();
        assert!(!gpus.is_empty(), "GPU list should not be empty");
    }

    #[test]
    fn test_get_packages_not_empty() {
        let pkgs = get_packages_info();
        assert!(!pkgs.is_empty(), "packages should not be empty");
    }

    #[test]
    fn test_get_desktop_not_empty() {
        let de = get_desktop_info();
        assert!(!de.is_empty(), "desktop info should not be empty");
    }
}
