use prettytable::{format, row, Table};
use sysinfo::{System, SystemExt};

mod ansi;
mod cli;
mod logo;
mod scanner;

fn main() {
    let ctx = cli::Ctx::new();
    generate_info(ctx);
}

fn generate_info(ctx: cli::Ctx) {
    // "new_all" used to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled
    let mut sys = System::new_all();

    // Update all information of `System` struct.
    sys.refresh_all();

    // Fetch system information:
    let user_prompt = scanner::get_user_prompt(&sys, &ctx);
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
    let gpu_name = scanner::get_gpu_name(&sys);
    let mem_info = scanner::get_mem_info(&sys);
    let palette = scanner::get_palette();
    let palette_lines = palette.as_slice().iter();

    let logo = scanner::get_logo(&sys);
    let logo_col = logo.unwrap_or_else(|| "".into());
    let logo_width = logo_col.lines().map(|s| ansi::len(s)).max().unwrap_or(0);
    let term_width = termsize::get().map(|w| w.cols).unwrap_or(80);
    let info_width = usize::from(term_width)
        .saturating_sub(logo_width)
        .saturating_sub(4); // includes width of column border plus 1 extra

    // Structure and output system information
    let sys_info_col = if !ctx.args.minimal {
        [
            Some("\n".to_owned()),
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
        .map(|s| ansi::truncate(&s, info_width))
        .collect::<Vec<String>>()
        .join("\n")
    } else {
        vec![user_prompt]
            .into_iter()
            .flatten()
            .collect::<Vec<String>>()
            .join("\n")
    };

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_CLEAN);

    // Check ctx and generate according tables
    if ctx.args.no_logo {
        table.add_row(row![&sys_info_col]);
    } else if ctx.args.minimal {
        table.add_row(row![&logo_col]);
        table.add_row(row![&sys_info_col]);
    } else if ctx.args.logo_only {
        table.add_row(row![&logo_col]);
    } else {
        table.add_row(row![&logo_col, &sys_info_col]);
    }
    table.printstd();
}
