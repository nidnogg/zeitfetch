use sysinfo::{System, SystemExt};
use std::process::Command;
use std::process::Stdio;
use std::io::{Read, Write};
fn main() {
    generate_info();
    
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
 
    // get a way to find use .contains() from 
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
                
                println!("{}", get_mac_friendly_name(sys_friendly_num_no_whitespace));
            }
        },   
        None => println!("N/A"),    
    }

    // Display system information:
    println!("System name:             {:?}", sys.name());
    println!("System kernel version:   {:?}", sys.kernel_version());
    println!("System OS version:       {:?}", sys.os_version());
    println!("System host name:        {:?}", sys.host_name());
    
    // Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());
    
    // Memory info
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

    match &*ver_num {
        "10.11.6" => String::from("OS X El Capitan"),
        &_ => String::from(""),
    }

}