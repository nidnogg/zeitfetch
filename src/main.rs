use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
fn main() {
    generate_info();
    
}

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

    println!("=> system:");
    // RAM and swap information:
    let total_memory = sys.total_memory().to_string();
    let used_memory = sys.used_memory().to_string();
    // let total_memory_slice: &str = &total_memory;
    println!("total memory: {} KB", &total_memory[..4]);
    println!("used memory : {} KB", &used_memory[..4]);
    // println!("total swap  : {} KB", sys.total_swap());
    // println!("used swap   : {} KB", sys.used_swap());

    // display system information:
    println!("System name:             {:?}", sys.name());
    println!("System kernel version:   {:?}", sys.kernel_version());
    println!("System OS version:       {:?}", sys.os_version());
    println!("System host name:        {:?}", sys.host_name());

    // Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());

    // // Display processes ID, name and disk usage:
    // for (pid, process) in sys.processes() {
    //     println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
    // }
}
