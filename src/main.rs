use sysinfo::{System, SystemExt};
use home;
use std::process::Command;
use std::process::Stdio;
use std::io::{Read, Write};
fn main() {
    generate_info();
    
}

fn generate_info() {
    // "new_all" used to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled
    let mut sys = System::new_all();

    // Update all information of `System` struct.
    sys.refresh_all();

    // Display system information:

    sys = get_user_prompt(sys);
    sys = get_sys_name(sys);
    sys = get_host_name(sys);
    sys = get_uptime(sys);
    sys = get_kernel(sys);
    sys = get_os_ver(sys);
    
    // Number of CPUs:
    println!("\x1b[93;1m{}\x1b[0m: {}", "NB CPUs", sys.cpus().len());
    
    // Memory info
    get_mem_info(sys);

    // A way to extract value from an Option() - if it's None, output is blank
    // if let Some(value) = sys.name() {
    //     println!("System name {}", value);
    // }
    
}

fn get_user_prompt(sys: System) -> System {
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
                let user_prompt = format!("\x1b[93;1m{}\x1b[0m@\x1b[93;1m{}\x1b[0m", username, host_name);
                // Extra 1 for @ character
                let total_width = host_name.len() + username.len() + 1; 
                let linebreak = std::iter::repeat("-").take(total_width).collect::<String>();
                println!("{}", user_prompt);
                println!("{}", linebreak);
            }
        }
    }
    sys
}

fn get_kernel(sys: System) -> System {
    if let Some(value) = sys.kernel_version() {
        println!("\x1b[93;1m{}\x1b[0m: {}", "Kernel", value);
        sys
    } else {
        println!("\x1b[93;1m{}\x1b[0m: N/A", "Kernel");
        sys 
    }
}

fn get_host_name(sys: System) -> System {
    if let Some(value) = sys.host_name() {
        println!("\x1b[93;1m{}\x1b[0m: {}", "Host", value);
        sys
    } else {
        println!("N/A");
        sys
    }
}

fn get_sys_name(sys: System) -> System {
    match sys.name() {
        Some(value) => {
            let sys_name = value; 
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
          
                println!("\x1b[93;1m{}\x1b[0m: {}", "OS", get_mac_friendly_name(sys_friendly_num_no_whitespace));
                sys
            } else {
                if let Some(os_ver) = sys.os_version() {
                    println!("\x1b[93;1m{}\x1b[0m: {} {}", "OS", sys_name, os_ver);
                    sys
                } else {
                    println!("\x1b[93;1m{}\x1b[0m: {}", "OS", sys_name);
                    sys
                }
            }
        },   
        None => {
            println!("N/A");
            sys
        },    
    }
}

fn get_uptime(sys: System) -> System {
    let mut uptime: f64 = sys.uptime() as f64;
    let days: f64 = uptime / (24.0 * 3600.0);
    if days > 1.0 {
        uptime = uptime % (24.0 * 3600.0);
        let hours: f64 = uptime / 3600.0;
        if hours > 1.0 {
            uptime = uptime % 3600.0;
            let minutes: f64 = uptime / 60.0;
            if minutes >= 1.0 {
                println!("\x1b[93;1m{}\x1b[0m: {} day(s), {} hour(s) and {} minute(s)", "Uptime", days.floor(), hours.floor(), minutes.floor());
            } else {
                println!("\x1b[93;1m{}\x1b[0m: {} day(s) {} hour(s)", "Uptime", days.floor(), hours.floor());
            }
        } else {
            let minutes: f64 = uptime / 60.0;
            if minutes >= 1.0 {
                println!("\x1b[93;1m{}\x1b[0m: {} day(s) and {} minute(s)", "Uptime", days.floor(), minutes.floor());
            } else {
                println!("\x1b[93;1m{}\x1b[0m: {} day(s)", "Uptime", days.floor());
            }
        }
    } else {
        let hours: f64 = uptime / 3600.0;
        if hours > 1.0 {
            uptime = uptime % 3600.0;
            let minutes: f64 = uptime / 60.0;
            if minutes >= 1.0 {
                println!("\x1b[93;1m{}\x1b[0m: {} hour(s) and {} minute(s)", "Uptime", hours.floor(), minutes.floor());
            } else {
                println!("\x1b[93;1m{}\x1b[0m: {} hour(s)", "Uptime", hours.floor());
            }
        } else {
            let minutes: f64 = uptime / 60.0;
            if minutes >= 1.0 {
                println!("\x1b[93;1m{}\x1b[0m: {} minute(s)", "Uptime", minutes.floor());
            } else {
                println!("\x1b[93;1m{}\x1b[0m: {} second(s)", "Uptime", uptime.floor());
            }
        }
    }
    sys
}

fn get_os_ver(sys: System) -> System {
    if let Some(os_ver) = sys.os_version() {
        println!("\x1b[93;1m{}\x1b[0m: {}", "System OS version", os_ver);
    }
    sys
}

// Warning - if sys is needed after get_mem_info, return it like in get_sys_name().
fn get_mem_info(sys: System) {
    // RAM information (non swap):
    const KB_TO_MIB: f64 = 0.00095367431640625;
    let total_memory = sys.total_memory() as f64 * KB_TO_MIB;
    let used_memory = sys.used_memory() as f64 * KB_TO_MIB;
    println!("\x1b[93;1m{}\x1b[0m: {}/{} MiB", "Memory", used_memory.floor(), total_memory.floor());
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