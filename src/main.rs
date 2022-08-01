use sysinfo::{System, SystemExt};
use std::process::Command;
use std::process::Stdio;
fn main() {
    generate_info();
    
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// fn output_sysname(System sys) {


// }
fn generate_info() {
    // "new_all" used to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled
    let mut sys = System::new_all();

    // update all information of our `System` struct.
    sys.refresh_all();

    // // display all disks' information:
    // println!("=> disks:");
    // for disk in sys.disks() {
    //     println!("{:?}", disk);
    // }

    // RAM and swap information:
    let total_memory = sys.total_memory().to_string();
    let used_memory = sys.used_memory().to_string();
    // let total_memory_slice: &str = &total_memory;
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


    // get a way to find use .contains() from 
    match sys.name() {
        Some(value) => {
            let sys_name = value; 
            if sys_name.contains("Darwin") {
                let os_num = Command::new("sw_vers")
                    .output()
                    .expect("Failed to fetch OS data (Mac)");
                    let sw_vers = String::from_utf8(os_num.stdout).unwrap();               
                    let sys_name_grep = Command::new("grep")
                        .arg(sw_vers)
                        .arg("ProductVersion")
                        .output()
                        .expect("Failed to trim sw_vers output for Mac friendly name");

                    let sys_friendly_name = String::from_utf8(sys_name_grep.stdout).unwrap();
                    println!("{}", sys_friendly_name);
           
            }
        },   
        None => println!("N/A"),
        
    }
    // if sys.name().to_string().contains("Darwin") {
    //     let os_num = Command::new("sw_vers")
    //         .stdout(Stdio::piped()).spawn()
    //         .expect("Failed to fetch OS data (Mac)");

    //     const os_version = os_num.stdout;
    //     println!(os_num);
    //     // Command::new("grep")
    //     //     .arg("")
        
    // }

    // println!("total swap  : {} KB", sys.total_swap());
    // println!("used swap   : {} KB", sys.used_swap());
    
    // display system information:
    println!("System name:             {:?}", sys.name());
    println!("System kernel version:   {:?}", sys.kernel_version());
    println!("System OS version:       {:?}", sys.os_version());
    println!("System host name:        {:?}", sys.host_name());
    
    // Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());
    
    // Memory info
    println!("Memory: {}/{} MiB", &used_memory[..4], &total_memory[..4]);
    // // Display processes ID, name and disk usage:
    // for (pid, process) in sys.processes() {
    //     println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
    // }
}
