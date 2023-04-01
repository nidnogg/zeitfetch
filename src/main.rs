use sysinfo::{System, SystemExt};

use prettytable::{format, row, Table};

mod logo;
mod scanner;
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

    // Fetch system information:

    let user_prompt = scanner::get_user_prompt(&sys);
    let sys_name = scanner::get_sys_name(&sys);
    let host_name = scanner::get_host_name(&sys);
    let uptime = scanner::get_uptime(&sys);
    let kernel = scanner::get_kernel(&sys);
    let os_ver = scanner::get_os_ver(&sys);
    let cpu_num = Some(format!(
        "\x1b[93;1m{}\x1b[0m: {}",
        "NB CPUs",
        sys.cpus().len()
    ));
    let cpu_name = scanner::get_cpu_name(&sys);
    // let gpu_name = scanner::get_gpu_name(&sys);
    let mem_info = scanner::get_mem_info(&sys);
    let palette = scanner::get_palette();
    let logo = scanner::get_logo(&sys);

    // Structure and output system information

    let sys_info_col = vec![
        Some("\n".to_owned()),
        user_prompt,
        sys_name,
        host_name,
        uptime,
        kernel,
        os_ver,
        cpu_num,
        cpu_name,
        // gpu_name,
        mem_info,
        palette,
    ]
    .into_iter()
    .filter_map(|x| x)
    .collect::<Vec<String>>()
    .join("\n");

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);
    table.add_row(row![&logo.unwrap_or_else(|| "".into()), &sys_info_col]);
    table.printstd();
}
