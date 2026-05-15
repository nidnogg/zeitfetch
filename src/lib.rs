pub mod scanner;
pub mod cli;
pub mod logo;
pub mod ansi;
pub mod platform_util;
pub mod table;

use sysinfo::{System, SystemExt};

/// Stripped system snapshot — no ANSI escape codes, just clean data.
pub struct SystemSnapshot {
    pub os_name: Option<String>,
    pub hostname: Option<String>,
    pub kernel: Option<String>,
    pub os_version: Option<String>,
    pub uptime: Option<String>,
    pub cpu_name: Option<String>,
    pub cpu_count: usize,
    pub gpu_name: Option<String>,
    pub memory_info: Option<String>,
}

/// Strip ANSI escape sequences from a string.
fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Skip until 'm' (end of ANSI escape)
            while let Some(&nc) = chars.peek() {
                chars.next();
                if nc == 'm' {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// Strip the "Label: " prefix that zeitfetch prepends.
fn strip_label(s: &str) -> String {
    match s.find(": ") {
        Some(i) => s[i + 2..].to_string(),
        None => s.to_string(),
    }
}

fn clean(s: &str) -> String {
    strip_label(&strip_ansi(s))
}

impl SystemSnapshot {
    /// Collect a fresh system snapshot using zeitfetch's scanner.
    pub fn collect() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        SystemSnapshot {
            os_name: scanner::get_sys_name(&sys).map(|s| clean(&s)),
            hostname: scanner::get_host_name(&sys).map(|s| clean(&s)),
            kernel: scanner::get_kernel(&sys).map(|s| clean(&s)),
            os_version: scanner::get_os_ver(&sys).map(|s| clean(&s)),
            uptime: scanner::get_uptime(&sys).map(|s| clean(&s)),
            cpu_name: scanner::get_cpu_name(&sys).map(|s| clean(&s)),
            cpu_count: sys.cpus().len(),
            gpu_name: scanner::get_gpu_name(&sys).map(|s| clean(&s)),
            memory_info: scanner::get_mem_info(&sys).map(|s| clean(&s)),
        }
    }
}
