use sysinfo::{System, SystemExt};
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

    sys = get_sys_name(sys);
    sys = get_host_name(sys);

    // TO-DO properly convert seconds to minutes, hours, days;
    println!("Uptime: {:?}", sys.uptime());
    sys = get_kernel(sys);
    println!("System OS version:       {:?}", sys.os_version());
    
    // Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());
    
    // Memory info
    get_mem_info(sys);

    // A way to extract value from an Option() - if it's None, output is blank
    // if let Some(value) = sys.name() {
    //     println!("System name {}", value);
    // }
    
}

fn get_kernel(sys: System) -> System {
    if let Some(value) = sys.kernel_version() {
        println!("Kernel: {}", value);
        return sys;
    } else {
        println!("Kernel: N/A");
        return sys; 
    }
}

fn get_host_name(sys: System) -> System {
    if let Some(value) = sys.host_name() {
        println!("Host: {}", value);
        return sys;
    } else {
        println!("N/A");
        return sys;
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
          
                println!("OS: {}", get_mac_friendly_name(sys_friendly_num_no_whitespace));
                return sys;
            } else {
                if let Some(os_ver) = sys.os_version() {
                    println!("OS: {} {}", sys_name, os_ver);
                    return sys;
                } else {
                    println!("OS: {}", sys_name);
                    return sys;
                }
            }
        },   
        None => {
            println!("N/A");
            return sys;
        },    
    }
}

// Warning - if sys is needed after get_mem_info, return it like in get_sys_name().
fn get_mem_info(sys: System) {
    // RAM information (non swap):
    let total_memory = sys.total_memory().to_string();
    let used_memory = sys.used_memory().to_string();

    println!("Memory: {}/{} MiB", &used_memory[..4], &total_memory[..4]);
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