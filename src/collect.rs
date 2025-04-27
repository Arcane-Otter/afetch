use std::fs;
use std::process::Command;

pub fn get_username() -> String {
    whoami::username()
}

pub fn get_hostname() -> String {
    whoami::fallible::hostname().expect("REASON")
}

pub fn get_distro() -> String {
    whoami::distro()
}

#[cfg(target_os = "linux")]
pub fn get_host() -> String {
    let vendor = fs::read_to_string("/sys/class/dmi/id/sys_vendor");
    let product = fs::read_to_string("/sys/class/dmi/id/product_name");
    format!("{} {}", vendor.expect("REASON").trim(), product.expect("REASON").trim())
}

#[cfg(target_os = "linux")]
pub fn get_kernel_version() -> String {
    let proc_version = fs::read_to_string("/proc/version");
    proc_version.expect("REASON").split_whitespace().nth(2).unwrap_or("Unknown").to_string()
}

#[cfg(target_os = "linux")]
pub fn get_uptime() -> f64 {
    let uptime = fs::read_to_string("/proc/uptime");
    uptime.expect("REASON").split_whitespace().next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0)
}

pub fn get_formatted_uptime(secs: f64) -> String {
    let days = secs / 86400.0;
    let hours = (secs % 86400.0) / 3600.0;
    let mins = (secs % 3600.0) / 60.0;

    match (days, hours, mins) {
        (0.0, 0.0, m) => format!("{:.0} Minutes", m),
        (0.0, h, m) => format!("{:.0} Hours {:.0} Minutes", h, m),
        (d, h, m) => format!("{:.0} Days {:.0} Hours {:.0} Minutes", d, h, m),
    }
}

// Check if an app/command is installed
pub fn is_app_installed(app: &str) -> bool {
    return Command::new("which").arg(app).output().map(|output| output.status.success()).unwrap_or(false);
}

// Count packages installed through apt
pub fn count_apt_packages() -> usize {
    if !is_app_installed("dpkg") {
        return 0;
    }
    let output = Command::new("dpkg-query").args(&["-f", ".\n", "-W"]).output();
    let count = String::from_utf8(output.unwrap().stdout).unwrap().lines().count();
    count
}

// Count packages installed through dnf
pub fn count_dnf_packages() -> usize {
    if !is_app_installed("dnf") {
        return 0;
    }
    let output = Command::new("dnf").args(&["list", "installed", "--quiet"]).output();
    let count = String::from_utf8(output.unwrap().stdout).unwrap().lines().count();
    count
}

// Count packages installed through pacman
pub fn count_pacman_packages() -> usize {
    if !is_app_installed("pacman") {
        return 0;
    }
    let output = Command::new("pacman").args(&["-Q"]).output();
    let count = String::from_utf8(output.unwrap().stdout).unwrap().lines().count();
    count
}

// Count packages installed through snap
pub fn count_snap_packages() -> usize {
    if !is_app_installed("snap") {
        return 0;
    }
    let output = Command::new("snap").args(&["list"]).output();
    let count = String::from_utf8(output.unwrap().stdout).unwrap().lines().count();
    count
}

#[cfg(target_os = "linux")]
pub fn get_formatted_package_count() -> String {
    let apt_packages = count_apt_packages();
    let dnf_packages = count_dnf_packages();
    let pacman_packages = count_pacman_packages();
    let snap_packages = count_snap_packages();
    format!("{}{}{}{}",
        if apt_packages > 0 { format!("apt:{} ", apt_packages) } else { String::new() },
        if dnf_packages > 0 { format!("dnf:{} ", dnf_packages) } else { String::new() },
        if pacman_packages > 0 { format!("pacman:{} ", pacman_packages) } else { String::new() },
        if snap_packages > 0 { format!("snap:{} ", snap_packages) } else { String::new() }).to_string()
}

pub fn get_shell_name() -> String {
    std::env::var("SHELL").ok()
        .and_then(|path| {
            std::path::Path::new(&path)
                .file_name()
                .and_then(|name| name.to_str())
                .map(|s| s.to_string())
        }).unwrap_or("[shell]".to_string())
}

#[cfg(target_os = "linux")]
pub fn get_shell_version(name: &str) -> String {
    let output = match Command::new(name).arg("--version").output() {
        Ok(output) => output,
        Err(_e) => return "[version]".to_string(),
    };
    match String::from_utf8(output.stdout) {
        Ok(version) => {
            version.lines()
                .next()
                .map(|line| line.trim().to_string())
                .unwrap_or_else(|| "[version]".to_string())
        }
        Err(_e) => return "[version]".to_string(),
    }
}

pub fn get_terminal_name() -> String {
    std::env::var("TERM").unwrap_or_else(|_| "Unknown Terminal".to_string())
    //"Kitty".to_string()
}

#[cfg(target_os = "linux")]
pub fn get_motherboard_vendor() -> String {
    let vendor = fs::read_to_string("/sys/class/dmi/id/board_vendor");
    vendor.expect("[vendor]").trim().to_string()
}

#[cfg(target_os = "linux")]
pub fn get_motherboard_name() -> String {
    let name = fs::read_to_string("/sys/class/dmi/id/board_name");
    name.expect("[name]").trim().to_string()
}

#[cfg(target_os = "linux")]
pub fn get_motherboard_info() -> String {
    let vendor = get_motherboard_vendor();
    let name = get_motherboard_name();
    format!("{} {}", vendor, name).to_string()
}

#[cfg(target_os = "linux")]
pub fn get_cpu_model() -> String {
    let cpuinfo = fs::read_to_string("/proc/cpuinfo");
    let cpuinfo_str = String::from_utf8(cpuinfo.unwrap().into())
        .expect("[cpuinfo]");
    return cpuinfo_str
        .lines()
        .nth(4).expect("[cpu model]")
        .split(':')
        .last()
        .unwrap()
        .trim()
        .to_string();
}

pub fn get_gpu_info() -> Vec<String> {
    let output = Command::new("lspci").args(&["-nn", "-v"]).output();
    let output_str = String::from_utf8(output.unwrap().stdout);
    let gpus_bind: String = output_str.expect("[gpus]");
    let gpus = gpus_bind
        .lines()
        .filter(|line| line.contains("VGA") || line.contains("3D controller"))
        .map(|s| s.to_string())
        .collect();
    return gpus;
}

#[cfg(target_os = "linux")]
pub fn get_memory_info() -> String {
    let meminfo = fs::read_to_string("/proc/meminfo");
    let lines = String::from_utf8(meminfo.unwrap().into())
        .expect("[meminfo]");

    let total: i64 = lines.lines().next().expect("[total]").split_whitespace().nth(1).unwrap().parse().unwrap();
    let free: i64 = lines.lines().nth(1).expect("[free]").split_whitespace().nth(1).unwrap().parse().unwrap();

    format!("{:.2}MB/{:.2}MB", (total - free) / 1024, total / 1024).to_string()
}
