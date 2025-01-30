use std::str;
use sysinfo::{System, SystemExt};
// use prettytable::{format, row, Table};

mod ansi;
mod cli;
mod logo;
mod platform_util;
mod scanner;
mod table;

use crate::table::{Table, format};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = cli::Ctx::new();
    generate_info(&ctx)?;

    Ok(())
}

fn generate_info(ctx: &cli::Ctx) -> Result<(), Box<dyn std::error::Error>> {
    // Force-enable proper ANSI color rendering on Windows Console
    _ = platform_util::enforce_ansi_windows();
    // "new_all" used to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled
    let mut sys = System::new_all();

    // Update all information of `System` struct.
    sys.refresh_all();

    let logo = scanner::get_logo(&sys);
    let logo_col = logo.unwrap_or_else(|| "".into());

    let mut table = Table::new();
    table.set_format(format::consts::FORMAT_CLEAN);

    // Check ctx and generate according tables
    if ctx.args.no_logo {
        let sys_info_col = full_sys_info_col(&sys, ctx);
        table.add_row(row![&sys_info_col]);
    } else if ctx.args.minimal {
        let sys_info_col = minimal_sys_info_col(&sys, ctx);
        table.add_row(row![&logo_col]);
        table.add_row(row![&sys_info_col]);
    } else if ctx.args.logo_only {
        table.add_row(row![&logo_col]);
    } else {
        let sys_info_col = full_sys_info_col(&sys, ctx);
        table.add_row(row![&logo_col, &sys_info_col]);
    }
    let mut buf = Vec::new();
    table.print(&mut buf)?;

    // Ansi truncating.
    // TO-DO debug why termsize is unable to set width with cargo build --release.
    // See https://github.com/nidnogg/zeitfetch/issues/14 for more details.
    // println!("{:?}", ctx.width);
    str::from_utf8(&buf)?
        .lines()
        .map(|s| match ctx.width {
            Some(width) => ansi::truncate(s, width),
            None => s.to_owned(),
        })
        .for_each(|s| println!("{s}"));

    Ok(())
}

fn full_sys_info_col(sys: &System, ctx: &cli::Ctx) -> String {
    let user_prompt = scanner::get_user_prompt(sys, ctx);
    let sys_name = scanner::get_sys_name(sys);
    let host_name = scanner::get_host_name(sys);
    let uptime = scanner::get_uptime(sys);
    let kernel = scanner::get_kernel(sys);
    let os_ver = scanner::get_os_ver(sys);
    let cpu_num = Some(format!(
        "\x1b[93;1m{}\x1b[0m: {}",
        "NB CPUs",
        sys.cpus().len()
    ));
    let cpu_name = scanner::get_cpu_name(sys);
    let gpu_name = scanner::get_gpu_name(sys);
    let mem_info = scanner::get_mem_info(sys);
    let palette = scanner::get_palette();
    let palette_lines = palette.as_slice().iter();

    [
        user_prompt,
        sys_name,
        host_name,
        uptime,
        kernel,
        os_ver,
        cpu_num,
        cpu_name,
        gpu_name,
        mem_info,
    ]
    .iter()
    .flatten()
    .chain(palette_lines)
    .map(|s| s.to_owned())
    .collect::<Vec<String>>()
    .join("\n")
}

fn minimal_sys_info_col(sys: &System, ctx: &cli::Ctx) -> String {
    scanner::get_user_prompt(sys, ctx).unwrap_or_else(|| "".into())
}
