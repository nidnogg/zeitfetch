use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::process::Command;
use std::process::Stdio;
use std::str;
use sysinfo::{CpuExt, System, SystemExt};

use crate::logo::*;

/// Data for a single display from the `system_profiler` command
#[derive(Serialize, Deserialize, Debug)]
struct SPDisplay {
    sppci_model: String,
}

/// Output of macOS's `system_profiler SPDisplayDataType` command
#[derive(Serialize, Deserialize, Debug)]
struct SPDisplaysDataTypeOutput {
    #[serde(rename = "SPDisplaysDataType")]
    sp_displays_data_type: Vec<SPDisplay>,
}

lazy_static! {
    static ref GPU_RE: Regex =
        Regex::new(r#"^\S+? "(VGA|3D|Display).*?" ".*?" "(?P<gpu>.*?)""#).unwrap();
}

pub fn get_logo(sys: &System) -> Option<String> {
    sys.name().map(|sys_name| {
        if sys_name.contains("Windows") {
            if let Some(kernel) = sys.kernel_version() {
                let float_kernel = kernel.parse::<i32>().unwrap();
                if float_kernel > 22000 {
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
        } else if sys_name.contains("Red Hat") {
            get_logo_by_distro("redhat")
        } else if sys_name.contains("Darwin") || sys_name.contains("Mac") {
            get_logo_by_distro("mac")
        } else {
            get_logo_by_distro("linux")
        }
    })
}

pub fn get_user_prompt(sys: &System) -> Option<String> {
    match (sys.name(), home::home_dir(), sys.host_name()) {
        (Some(os), Some(home_dir), Some(host_name)) => {
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
            let user_prompt = format!(
                "\x1b[93;1m{}\x1b[0m@\x1b[93;1m{}\x1b[0m",
                username, host_name
            );
            // Extra 1 for @ character
            let total_width = host_name.len() + username.len() + 1;
            let linebreak = "-".repeat(total_width);
            let final_user_prompt = format!("{}\n{}", user_prompt, linebreak);
            Some(final_user_prompt)
        }
        _ => None,
    }
}

pub fn get_kernel(sys: &System) -> Option<String> {
    if let Some(value) = sys.kernel_version() {
        Some(format!("\x1b[93;1m{}\x1b[0m: {}", "Kernel", value))
    } else {
        Some("\x1b[93;1mKernel\x1b[0m".into())
    }
}

pub fn get_host_name(sys: &System) -> Option<String> {
    sys.host_name()
        .map(|value| format!("\x1b[93;1m{}\x1b[0m: {}", "Host", value))
}

pub fn get_sys_name(sys: &System) -> Option<String> {
    sys.name().map(|sys_name| {
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
            let sys_friendly_num_no_whitespace = &sys_friendly_num[..sys_friendly_num.len() - 1];

            let final_sys_name = format!(
                "\x1b[93;1m{}\x1b[0m: {}",
                "OS",
                get_mac_friendly_name(String::from(sys_friendly_num_no_whitespace))
            );
            final_sys_name

        // Windows 11
        } else if sys_name.contains("Windows") {
            // let final_sys_name = format!("\x1b[93;1m{}\x1b[0m: {} 11", "OS", sys_name);
            if let Some(kernel) = sys.kernel_version() {
                let float_kernel = kernel.parse::<i32>().unwrap();
                if float_kernel > 22000 {
                    let final_sys_name = format!("\x1b[93;1m{}\x1b[0m: {}", "OS", "Windows 11");
                    final_sys_name
                } else {
                    let final_sys_name = format!("\x1b[93;1m{}\x1b[0m: {}", "OS", "Windows 10");
                    final_sys_name
                }
            } else {
                let final_sys_name = format!("\x1b[93;1m{}\x1b[0m", "Windows");
                final_sys_name
            }
        } else {
            match sys.os_version() {
                Some(os_ver) => {
                    let final_sys_name =
                        format!("\x1b[93;1m{}\x1b[0m: {} {}", "OS", sys_name, os_ver);
                    final_sys_name
                }
                None => {
                    let final_sys_name = format!("\x1b[93;1m{}\x1b[0m: {}", "OS", sys_name);
                    final_sys_name
                }
            }
        }
    })
}

pub fn get_uptime(sys: &System) -> Option<String> {
    let mut uptime: f64 = sys.uptime() as f64;
    let days: f64 = uptime / (24.0 * 3600.0);
    Some(if days > 1.0 {
        uptime %= 24.0 * 3600.0;
        let hours: f64 = uptime / 3600.0;
        if hours > 1.0 {
            uptime %= 3600.0;
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
            uptime %= 3600.0;
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
    })
}

pub fn get_os_ver(sys: &System) -> Option<String> {
    sys.os_version()
        .map(|os_ver| format!("\x1b[93;1m{}\x1b[0m: {}", "System OS version", os_ver))
}

pub fn get_cpu_name(sys: &System) -> Option<String> {
    let cpu_brand = sys.cpus()[0].brand();
    let cpu_frequency = if cpu_brand.contains("Apple M") {
        String::from("")
    } else {
        format!("@ {} GHz", sys.cpus()[0].frequency())
    };
    let full_cpu_name = format!(
        "\x1b[93;1m{}\x1b[0m: {} {}",
        "CPU",
        cpu_brand.trim(),
        cpu_frequency
    );
    Some(full_cpu_name)
}

pub fn get_gpu_name(sys: &System) -> Option<String> {
    // works on wsl, needs formatting and search on linux
    sys.name().and_then(|sys_name| {
        let gpus = if sys_name.contains("Windows") {
            let win_fetch_gpu = Command::new("wmic")
                .args(["path", "win32_VideoController", "get", "name"])
                .output()
                .ok()?;
            let gpu_name_buf = win_fetch_gpu.stdout;
            let processed_gpu_name = match str::from_utf8(&gpu_name_buf) {
                Ok(result) => result,
                Err(_) => return None,
            };
            let trimmed_gpu_name: String = processed_gpu_name
                .chars()
                .take(0)
                .chain(processed_gpu_name.chars().skip(4))
                .collect();
            vec![trimmed_gpu_name.trim().into()]
        } else if sys_name.contains("Darwin") || sys_name.contains("Mac") {
            let cmd = Command::new("system_profiler")
                .args(["-json", "SPDisplaysDataType"])
                .stdout(Stdio::piped())
                .spawn()
                .ok()?;
            parse_system_profiler_json_output(std::io::BufReader::new(cmd.stdout?))
        } else {
            // Linux
            let lspci = Command::new("lspci")
                .args(["-mm"])
                .stdout(Stdio::piped())
                .spawn()
                .ok()?;
            parse_lspci_mm_output(lspci.stdout?)
        };

        match gpus.as_slice() {
            [gpu] => Some(format!("\x1b[93;1m{}\x1b[0m: {}", "GPU", gpu)),
            gpus => Some(
                gpus.iter()
                    .enumerate()
                    .map(|(i, gpu)| format!("\x1b[93;1m{} #{}\x1b[0m: {}", "GPU", i + 1, gpu))
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
        }
    })
}

fn parse_lspci_mm_output(reader: impl Read) -> Vec<String> {
    use std::io::{BufRead, BufReader};
    BufReader::new(reader)
        .lines()
        .flat_map(|line| {
            line.ok().and_then(|line| {
                GPU_RE
                    .captures(&line)
                    .and_then(|cap| cap.name("gpu").map(|cap| cap.as_str().to_owned()))
            })
        })
        .collect()
}

fn parse_system_profiler_json_output(reader: impl Read) -> Vec<String> {
    let displays: SPDisplaysDataTypeOutput = match serde_json::from_reader(reader) {
        Ok(o) => o,
        Err(_) => return vec![],
    };
    displays
        .sp_displays_data_type
        .iter()
        .map(|d| d.sppci_model.to_owned())
        .collect::<Vec<_>>()
}

pub fn get_mem_info(sys: &System) -> Option<String> {
    // RAM information (non swap):
    const KB_TO_MIB: f64 = 0.00095367431640625 * 0.001;
    let total_memory = sys.total_memory() as f64 * KB_TO_MIB;
    let used_memory = sys.used_memory() as f64 * KB_TO_MIB;
    let mem_info = format!(
        "\x1b[93;1m{}\x1b[0m: {}/{} MiB",
        "Memory",
        used_memory.floor(),
        total_memory.floor()
    );
    Some(mem_info)
}

pub fn get_palette() -> Option<String> {
    let palette = "\n\
        \x1b[30m███\x1b[0m\x1b[31m███\x1b[0m\x1b[32m███\x1b[0m\x1b[33m███\x1b[0m\
        \x1b[34m███\x1b[0m\x1b[35m███\x1b[0m\x1b[36m███\x1b[0m\x1b[90;1m███\x1b[0m\n\
        \x1b[90;1m███\x1b[0m\x1b[91;1m███\x1b[0m\x1b[92;1m███\x1b[0m\x1b[93;1m███\x1b[0m\
        \x1b[94;1m███\x1b[0m\x1b[95;1m███\x1b[0m\x1b[96;1m███\x1b[0m\x1b[100;1m███\x1b[0m\
        "
    .to_string();
    Some(palette)
}

fn get_mac_friendly_name(untrimmed_ver_num: String) -> String {
    let ver_num = untrimmed_ver_num.trim();
    let friendly_name = match ver_num {
        ver if ver.starts_with("10.4") => "Mac OS X Tiger",
        ver if ver.starts_with("10.5") => "Mac OS X Leopard",
        ver if ver.starts_with("10.6") => "Mac OS X Snow Leopard",
        ver if ver.starts_with("10.7") => "Mac OS X Lion",
        ver if ver.starts_with("10.8") => "OS X Mountain Lion",
        ver if ver.starts_with("10.9") => "OS X Mavericks",
        ver if ver.starts_with("10.10") => "OS X Yosemite",
        ver if ver.starts_with("10.11") => "OS X El Capitan",
        ver if ver.starts_with("10.12") => "macOS Sierra",
        ver if ver.starts_with("10.13") => "macOS High Sierra",
        ver if ver.starts_with("10.14") => "macOS Mojave",
        ver if ver.starts_with("10.15") => "macOS Catalina",
        ver if ver.starts_with("10.16") => "macOS Big Sur",
        ver if ver.starts_with("11.") => "macOS Catalina",
        ver if ver.starts_with("12.") => "macOS Monterey",
        ver if ver.starts_with("13.") => "macOS Ventura",
        _ => "macOS",
    };

    format!("{} {}", friendly_name, ver_num)
}

#[cfg(test)]
mod tests {
    #[test]
    fn gpu_regex_empty() {
        use crate::scanner::GPU_RE;
        let cap = GPU_RE.captures("");
        assert!(cap.is_none());
    }

    #[test]
    fn gpu_regex_not_gpu() {
        use crate::scanner::GPU_RE;
        let cap = GPU_RE.captures(r#"00:00.0 "Host bridge" "Intel Corporation" "12th Gen Core Processor Host Bridge/DRAM Registers" -r02 -p00 "Dell" "Device 0b19""#);
        assert!(cap.is_none());
    }

    #[test]
    fn gpu_regex_nvidia() {
        use crate::scanner::GPU_RE;
        let cap = GPU_RE.captures(r#"01:00.0 "3D controller" "NVIDIA Corporation" "GA107M [GeForce RTX 3050 Ti Mobile]" -ra1 -p00 "Dell" "Device 0b19""#);
        let gpu = cap.and_then(|c| c.name("gpu").map(|m| m.as_str()));
        assert_eq!(gpu, Some("GA107M [GeForce RTX 3050 Ti Mobile]"));
    }

    #[test]
    fn parse_lspci_mm_output() {
        let input = r#"00:00.0 "Host bridge" "Intel Corporation" "12th Gen Core Processor Host Bridge/DRAM Registers" -r02 -p00 "Dell" "Device 0b19"
00:01.0 "PCI bridge" "Intel Corporation" "12th Gen Core Processor PCI Express x16 Controller #1" -r02 -p00 "Dell" "Device 0b19"
00:02.0 "VGA compatible controller" "Intel Corporation" "Alder Lake-P Integrated Graphics Controller" -r0c -p00 "Dell" "Device 0b19"
00:04.0 "Signal processing controller" "Intel Corporation" "Alder Lake Innovation Platform Framework Processor Participant" -r02 -p00 "Dell" "Device 0b19"
00:06.0 "PCI bridge" "Intel Corporation" "12th Gen Core Processor PCI Express x4 Controller #0" -r02 -p00 "Dell" "Device 0b19"
00:07.0 "PCI bridge" "Intel Corporation" "Alder Lake-P Thunderbolt 4 PCI Express Root Port #0" -r02 -p00 "Dell" "Device 0b19"
00:07.1 "PCI bridge" "Intel Corporation" "Alder Lake-P Thunderbolt 4 PCI Express Root Port #1" -r02 -p00 "Dell" "Device 0b19"
00:08.0 "System peripheral" "Intel Corporation" "12th Gen Core Processor Gaussian & Neural Accelerator" -r02 -p00 "Dell" "Device 0b19"
00:0d.0 "USB controller" "Intel Corporation" "Alder Lake-P Thunderbolt 4 USB Controller" -r02 -p30 "Dell" "Device 0b19"
00:0d.2 "USB controller" "Intel Corporation" "Alder Lake-P Thunderbolt 4 NHI #0" -r02 -p40 "Dell" "Device 0b19"
00:12.0 "Serial controller" "Intel Corporation" "Alder Lake-P Integrated Sensor Hub" -r01 -p00 "Dell" "Device 0b19"
00:14.0 "USB controller" "Intel Corporation" "Alder Lake PCH USB 3.2 xHCI Host Controller" -r01 -p30 "Dell" "Device 0b19"
00:14.2 "RAM memory" "Intel Corporation" "Alder Lake PCH Shared SRAM" -r01 -p00 "Dell" "Device 0b19"
00:14.3 "Network controller" "Intel Corporation" "Alder Lake-P PCH CNVi WiFi" -r01 -p00 "Intel Corporation" "Wi-Fi 6E AX211 160MHz"
00:15.0 "Serial bus controller" "Intel Corporation" "Alder Lake PCH Serial IO I2C Controller #0" -r01 -p00 "Dell" "Device 0b19"
00:15.1 "Serial bus controller" "Intel Corporation" "Alder Lake PCH Serial IO I2C Controller #1" -r01 -p00 "Dell" "Device 0b19"
00:16.0 "Communication controller" "Intel Corporation" "Alder Lake PCH HECI Controller" -r01 -p00 "Dell" "Device 0b19"
00:1c.0 "PCI bridge" "Intel Corporation" "Device 51bb" -r01 -p00 "Dell" "Device 0b19"
00:1f.0 "ISA bridge" "Intel Corporation" "Alder Lake PCH eSPI Controller" -r01 -p00 "Dell" "Device 0b19"
00:1f.3 "Audio device" "Intel Corporation" "Alder Lake PCH-P High Definition Audio Controller" -r01 -p80 "Dell" "Device 0b19"
00:1f.4 "SMBus" "Intel Corporation" "Alder Lake PCH-P SMBus Host Controller" -r01 -p00 "Dell" "Device 0b19"
00:1f.5 "Serial bus controller" "Intel Corporation" "Alder Lake-P PCH SPI Controller" -r01 -p00 "Dell" "Device 0b19"
01:00.0 "3D controller" "NVIDIA Corporation" "GA107M [GeForce RTX 3050 Ti Mobile]" -ra1 -p00 "Dell" "Device 0b19"
02:00.0 "Non-Volatile memory controller" "Samsung Electronics Co Ltd" "NVMe SSD Controller PM9A1/PM9A3/980PRO" -p02 "Samsung Electronics Co Ltd" "Device a801"
54:00.0 "PCI bridge" "Intel Corporation" "JHL7540 Thunderbolt 3 Bridge [Titan Ridge DD 2018]" -r06 -p00 "Intel Corporation" "Device 0000"
55:02.0 "PCI bridge" "Intel Corporation" "JHL7540 Thunderbolt 3 Bridge [Titan Ridge DD 2018]" -r06 -p00 "Intel Corporation" "Device 0000"
55:04.0 "PCI bridge" "Intel Corporation" "JHL7540 Thunderbolt 3 Bridge [Titan Ridge DD 2018]" -r06 -p00 "Intel Corporation" "Device 0000"
56:00.0 "USB controller" "Intel Corporation" "JHL7540 Thunderbolt 3 USB Controller [Titan Ridge DD 2018]" -r06 -p30 "Intel Corporation" "Device 0000"
a5:00.0 "Unassigned class [ff00]" "Realtek Semiconductor Co., Ltd." "RTS5260 PCI Express Card Reader" -r01 -p00 "Dell" "Device 0b19"
"#;
        assert_eq!(
            super::parse_lspci_mm_output(input.as_bytes()),
            vec![
                "Alder Lake-P Integrated Graphics Controller",
                "GA107M [GeForce RTX 3050 Ti Mobile]"
            ]
        );
    }

    #[test]
    fn parse_system_profiler_json_output() {
        let input = r#"{
  "SPDisplaysDataType" : [
    {
      "_name" : "kHW_AppleM1Item",
      "spdisplays_mtlgpufamilysupport" : "spdisplays_metal3",
      "spdisplays_ndrvs" : [
        {
          "_name" : "Color LCD",
          "_spdisplays_display-product-id" : "a047",
          "_spdisplays_display-serial-number" : "---redacted---",
          "_spdisplays_display-vendor-id" : "610",
          "_spdisplays_display-week" : "0",
          "_spdisplays_display-year" : "0",
          "_spdisplays_displayID" : "1",
          "_spdisplays_pixels" : "2880 x 1800",
          "_spdisplays_resolution" : "1440 x 900 @ 60.00Hz",
          "spdisplays_ambient_brightness" : "spdisplays_yes",
          "spdisplays_connection_type" : "spdisplays_internal",
          "spdisplays_display_type" : "spdisplays_built-in_retinaLCD",
          "spdisplays_main" : "spdisplays_yes",
          "spdisplays_mirror" : "spdisplays_off",
          "spdisplays_online" : "spdisplays_yes",
          "spdisplays_pixelresolution" : "spdisplays_2560x1600Retina"
        }
      ],
      "spdisplays_vendor" : "sppci_vendor_Apple",
      "sppci_bus" : "spdisplays_builtin",
      "sppci_cores" : "7",
      "sppci_device_type" : "spdisplays_gpu",
      "sppci_model" : "Apple M1"
    }
  ]
}
"#;
        assert_eq!(
            super::parse_system_profiler_json_output(input.as_bytes()),
            vec!["Apple M1"],
        );
    }
}
