use home;
use std::io::{Read, Write};
use std::process::Command;
use std::process::Stdio;
use std::str;
use sysinfo::{CpuExt, System, SystemExt};

use crate::logo::*;

pub fn get_logo(sys: &System) -> String {
    if let Some(sys_name) = sys.name() {
        if sys_name.contains("Windows") {
            if let Some(kernel) = sys.kernel_version() {
                if kernel.contains("22000") {
                    get_logo_by_distro("win11")
                } else {
                    get_logo_by_distro("win")
                }
            } else {
                get_logo_by_distro("win")
            }
        } else if sys_name.contains("Debian") {
            get_logo_by_distro("deb")
        } else if sys_name.contains("Ubuntu") {
            get_logo_by_distro("ubuntu")
        } else if sys_name.contains("Fedora") {
            get_logo_by_distro("fedora")
        } else if sys_name.contains("Arch") {
            get_logo_by_distro("arch")
        } else {
            get_logo_by_distro("linux")
        }
    } else {
        String::from("")
    }
}

pub fn get_user_prompt(sys: &System) -> String {
    if let Some(os) = sys.name() {
        if let Some(home_dir) = home::home_dir() {
            let path = String::from(home_dir.to_string_lossy());
            let mut count = 0;
            for letter in path.chars().rev() {
                if (letter == '\\' && os.contains("Windows")) || (letter == '/') {
                    break;
                }
                count += 1;
            }

            let start_user_index = path.len() - count;
            let username = &path[start_user_index..];

            if let Some(host_name) = sys.host_name() {
                let user_prompt = format!(
                    "\x1b[93;1m{}\x1b[0m@\x1b[93;1m{}\x1b[0m",
                    username, host_name
                );
                // Extra 1 for @ character
                let total_width = host_name.len() + username.len() + 1;
                let linebreak = std::iter::repeat("-").take(total_width).collect::<String>();
                let final_user_prompt = format!("{}\n{}", user_prompt, linebreak);
                final_user_prompt
            } else {
                String::from("N/A")
            }
        } else {
            String::from("N/A")
        }
    } else {
        String::from("N/A")
    }
}

pub fn get_kernel(sys: &System) -> String {
    if let Some(value) = sys.kernel_version() {
        let kernel = format!("\x1b[93;1m{}\x1b[0m: {}", "Kernel", value);
        kernel
    } else {
        let kernel = format!("\x1b[93;1m{}\x1b[0m: N/A", "Kernel");
        kernel
    }
}

pub fn get_host_name(sys: &System) -> String {
    if let Some(value) = sys.host_name() {
        let host_name = format!("\x1b[93;1m{}\x1b[0m: {}", "Host", value);
        host_name
    } else {
        let host_name = String::from("N/A");
        host_name
    }
}

pub fn get_sys_name(sys: &System) -> String {
    match sys.name() {
        Some(value) => {
            let sys_name = value;

            // Mac OS
            if sys_name.contains("Darwin") {
                let mut cmd_sw_vers = Command::new("sw_vers")
                    .stdout(Stdio::piped())
                    .spawn()
                    .unwrap();

                let mut cmd_grep = Command::new("grep")
                    .arg("ProductVersion")
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()
                    .unwrap();

                if let Some(ref mut stdout) = cmd_sw_vers.stdout {
                    if let Some(ref mut stdin) = cmd_grep.stdin {
                        let mut buf: Vec<u8> = Vec::new();
                        stdout.read_to_end(&mut buf).unwrap();
                        stdin.write_all(&buf).unwrap();
                    }
                }
                let res = cmd_grep.wait_with_output().unwrap().stdout;
                let sys_friendly_num = &String::from_utf8(res).unwrap()[16..];
                let sys_friendly_num_no_whitespace =
                    &sys_friendly_num[..sys_friendly_num.len() - 1];

                let final_sys_name = format!(
                    "\x1b[93;1m{}\x1b[0m: {}",
                    "OS",
                    get_mac_friendly_name(sys_friendly_num_no_whitespace)
                );
                final_sys_name

            // Windows 11
            } else if sys_name.contains("Windows") {
                // let final_sys_name = format!("\x1b[93;1m{}\x1b[0m: {} 11", "OS", sys_name);
                if let Some(kernel) = sys.kernel_version() {
                    if kernel.contains("22000") {
                        let final_sys_name = format!(
                            "\x1b[93;1m{}\x1b[0m: {}",
                            "OS",
                            "Windows 11"
                        );
                        final_sys_name 
                    } else {
                        let final_sys_name = format!(
                            "\x1b[93;1m{}\x1b[0m: {}",
                            "OS",
                            "Windows 10"
                        );
                        final_sys_name 
                    }
                } else {
                    let final_sys_name = format!(
                        "\x1b[93;1m{}\x1b[0m",
                        "Windows"
                    );
                    final_sys_name 
                }
            } else {
                if let Some(os_ver) = sys.os_version() {
                    let final_sys_name =
                        format!("\x1b[93;1m{}\x1b[0m: {} {}", "OS", sys_name, os_ver);
                    final_sys_name
                } else {
                    let final_sys_name = format!("\x1b[93;1m{}\x1b[0m: {}", "OS", sys_name);
                    final_sys_name
                }
            }
        }
        None => String::from("N/A"),
    }
}

pub fn get_uptime(sys: &System) -> String {
    let mut uptime: f64 = sys.uptime() as f64;
    let days: f64 = uptime / (24.0 * 3600.0);
    if days > 1.0 {
        uptime = uptime % (24.0 * 3600.0);
        let hours: f64 = uptime / 3600.0;
        if hours > 1.0 {
            uptime = uptime % 3600.0;
            let minutes: f64 = uptime / 60.0;
            if minutes >= 1.0 {
                let final_uptime = format!(
                    "\x1b[93;1m{}\x1b[0m: {} day(s), {} hour(s) and {} minute(s)",
                    "Uptime",
                    days.floor(),
                    hours.floor(),
                    minutes.floor()
                );
                final_uptime
            } else {
                let final_uptime = format!(
                    "\x1b[93;1m{}\x1b[0m: {} day(s) {} hour(s)",
                    "Uptime",
                    days.floor(),
                    hours.floor()
                );
                final_uptime
            }
        } else {
            let minutes: f64 = uptime / 60.0;
            if minutes >= 1.0 {
                let final_uptime = format!(
                    "\x1b[93;1m{}\x1b[0m: {} day(s) and {} minute(s)",
                    "Uptime",
                    days.floor(),
                    minutes.floor()
                );
                final_uptime
            } else {
                let final_uptime =
                    format!("\x1b[93;1m{}\x1b[0m: {} day(s)", "Uptime", days.floor());
                final_uptime
            }
        }
    } else {
        let hours: f64 = uptime / 3600.0;
        if hours > 1.0 {
            uptime = uptime % 3600.0;
            let minutes: f64 = uptime / 60.0;
            if minutes >= 1.0 {
                let final_uptime = format!(
                    "\x1b[93;1m{}\x1b[0m: {} hour(s) and {} minute(s)",
                    "Uptime",
                    hours.floor(),
                    minutes.floor()
                );
                final_uptime
            } else {
                let final_uptime =
                    format!("\x1b[93;1m{}\x1b[0m: {} hour(s)", "Uptime", hours.floor());
                final_uptime
            }
        } else {
            let minutes: f64 = uptime / 60.0;
            if minutes >= 1.0 {
                let final_uptime = format!(
                    "\x1b[93;1m{}\x1b[0m: {} minute(s)",
                    "Uptime",
                    minutes.floor()
                );
                final_uptime
            } else {
                let final_uptime = format!(
                    "\x1b[93;1m{}\x1b[0m: {} second(s)",
                    "Uptime",
                    uptime.floor()
                );
                final_uptime
            }
        }
    }
}

pub fn get_os_ver(sys: &System) -> String {
    if let Some(os_ver) = sys.os_version() {
        let final_os_ver = format!("\x1b[93;1m{}\x1b[0m: {}", "System OS version", os_ver);
        final_os_ver
    } else {
        String::from("N/A")
    }
}

pub fn get_cpu_name(sys: &System) -> String {
    let cpu_brand = sys.cpus()[0].brand();
    let cpu_frequency = sys.cpus()[0].frequency();
    let full_cpu_name = format!(
        "\x1b[93;1m{}\x1b[0m: {} @ {} GHz",
        "CPU",
        cpu_brand.trim(),
        cpu_frequency
    );
    full_cpu_name
}

pub fn get_gpu_name(sys: &System) -> String {
    // works on wsl, needs formatting and grep: lspci | grep -i --color 'vga\|3d\|2d'
    if let Some(sys_name) = sys.name() {
        // Windows
        if sys_name.contains("Windows") {
            let win_fetch_gpu = Command::new("wmic")
                .args(["path", "win32_VideoController", "get", "name"])
                .output()
                .expect("Failed to fetch Win32 GPU data");

            let gpu_name_buf = win_fetch_gpu.stdout;
            // wmic path win32_VideoController get name
            // nf uses get caption
            let processed_gpu_name = match str::from_utf8(&gpu_name_buf) {
                Ok(result) => result,
                Err(e) => panic!("Failed to process Win32 GPU data: {:?}", e),
            };

            let trimmed_gpu_name: String = processed_gpu_name
                .chars()
                .take(0)
                .chain(processed_gpu_name.chars().skip(4))
                .collect();

            let final_gpu_name = format!("\x1b[93;1m{}\x1b[0m: {}", "GPU", trimmed_gpu_name.trim());
            final_gpu_name
        // Linux
        } else {
            // lspci | grep -i --color 'vga\|3d\|2d'
            let mut cmd_lspci = Command::new("lspci")
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

            let mut cmd_grep = Command::new("grep")
                .args(["-i", "--color", "\'vga\\|3d\\|2d\'"])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

            if let Some(ref mut stdout) = cmd_lspci.stdout {
                if let Some(ref mut stdin) = cmd_grep.stdin {
                    let mut buf: Vec<u8> = Vec::new();
                    stdout.read_to_end(&mut buf).unwrap();
                    stdin.write_all(&buf).unwrap();
                }
            }

            let gpu_name_buf = cmd_grep.wait_with_output().unwrap().stdout;

            let processed_gpu_name = match str::from_utf8(&gpu_name_buf) {
                Ok(result) => result,
                Err(e) => panic!("Failed to process Win32 GPU data: {:?}", e),
            };

            // let sys_friendly_num = &String::from_utf8(res).unwrap()[16..];
            // let sys_friendly_num_no_whitespace = &sys_friendly_num[..sys_friendly_num.len() - 1];

            let mut processed_gpu_no_newline = String::from(processed_gpu_name);
            processed_gpu_no_newline.pop();
            let final_sys_name = format!(
                "\x1b[93;1m{}\x1b[0m: {}",
                "GPU",
                processed_gpu_no_newline
            );


            final_sys_name
        }
    } else {
        String::from("N/A")
    }
}

pub fn get_mem_info(sys: &System) -> String {
    // RAM information (non swap):
    const KB_TO_MIB: f64 = 0.00095367431640625;
    let total_memory = sys.total_memory() as f64 * KB_TO_MIB;
    let used_memory = sys.used_memory() as f64 * KB_TO_MIB;
    let mem_info = format!(
        "\x1b[93;1m{}\x1b[0m: {}/{} MiB",
        "Memory",
        used_memory.floor(),
        total_memory.floor()
    );
    mem_info
}

pub fn get_palette() -> String {
    let palette = format!(
        "\n\
        \x1b[30m███\x1b[0m\x1b[31m███\x1b[0m\x1b[32m███\x1b[0m\x1b[33m███\x1b[0m\
        \x1b[34m███\x1b[0m\x1b[35m███\x1b[0m\x1b[36m███\x1b[0m\x1b[90;1m███\x1b[0m\n\
        \x1b[90;1m███\x1b[0m\x1b[91;1m███\x1b[0m\x1b[92;1m███\x1b[0m\x1b[93;1m███\x1b[0m\
        \x1b[94;1m███\x1b[0m\x1b[95;1m███\x1b[0m\x1b[96;1m███\x1b[0m\x1b[100;1m███\x1b[0m\
        "
    );
    palette
}

fn get_mac_friendly_name(ver_num: &str) -> String {
    // ripped straight out of neofetch - move this into separate file (sys_lists.rs) probably
    // with helper function to return it
    // use match pennies example to return this.{}
    // case $osx_version in
    //             10.4*)  codename="Mac OS X Tiger" ;;
    //             10.5*)  codename="Mac OS X Leopard" ;;
    //             10.6*)  codename="Mac OS X Snow Leopard" ;;
    //             10.7*)  codename="Mac OS X Lion" ;;
    //             10.8*)  codename="OS X Mountain Lion" ;;
    //             10.9*)  codename="OS X Mavericks" ;;
    //             10.10*) codename="OS X Yosemite" ;;
    //             10.11*) codename="OS X El Capitan" ;;
    //             10.12*) codename="macOS Sierra" ;;
    //             10.13*) codename="macOS High Sierra" ;;
    //             10.14*) codename="macOS Mojave" ;;
    //             10.15*) codename="macOS Catalina" ;;
    //             10.16*) codename="macOS Big Sur" ;;
    //             11.*)  codename="macOS Big Sur" ;;
    //             12.*)  codename="macOS Monterey" ;;
    //             *)      codename=macOS ;;
    //         esac

    // TO-DO Figure out how to match starting substring
    match &*ver_num {
        "10.11.6" => String::from("OS X El Capitan"),
        &_ => String::from(""),
    }
}
