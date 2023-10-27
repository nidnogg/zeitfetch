use std::{env, process::exit};

use sysinfo::{System, SystemExt};

use prettytable::{format, row, Table};

mod logo;
mod scanner;
pub struct Ctx {
    args: Args,
}

#[derive(Debug, Default)]
pub struct Args {
    no_logo: bool,
    logo_only: bool,
    minimal: bool,
    exclusive_args: i32,
}

impl Ctx {
    fn new() -> Self {
        let args = Args::parse_args();
        Ctx { args }
    }
}

impl Args {
    fn parse_args() -> Self {
        let mut args = Args::default();
        let arg_vec: Vec<String> = env::args().collect();

        if arg_vec.contains(&String::from("--help")) || arg_vec.contains(&String::from("--h")) {
            let help_text = "zeitfetch\n\
            Usage:\n\
            zeitfetch [options]\n\
            \n\
            Options:\n\
            -h, --help      Show this help message\n\
            -v, --version   Show the version number\n\
            --no-logo    Display only system information\n\
            --logo-only  Display only distro logo\n\
            --minimal    Display logo and user prompt, vertically\n";
            println!("{}", help_text);
            exit(0);
        }

        if arg_vec.contains(&String::from("--version")) || arg_vec.contains(&String::from("--v")) {
            let version_text = env!("CARGO_PKG_VERSION");
            println!("zeitfetch v{}", version_text);
            exit(0);
        }

        env!("CARGO_PKG_VERSION");

        if arg_vec.contains(&String::from("--no-logo")) {
            args.no_logo = true;
            args.exclusive_args += 1;
        }

        if arg_vec.contains(&String::from("--logo-only")) {
            args.logo_only = true;
            args.exclusive_args += 1;
        }

        if arg_vec.contains(&String::from("--minimal")) {
            args.minimal = true;
            args.exclusive_args += 1;
        }

        if args.exclusive_args > 1 {
            println!("Please include only one of the following arguments:\n--no-logo, --logo-only, --minimal");
            exit(1);
        }

        args
    }
}

fn main() {
    let ctx = Ctx::new();
    generate_info(ctx);
}

fn generate_info(ctx: Ctx) {
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
    let logo = scanner::get_logo(&sys);

    // Structure and output system information
    let sys_info_col = if !ctx.args.minimal {
        vec![
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
            palette,
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<String>>()
        .join("\n")
    } else {
        vec![user_prompt]
            .into_iter()
            .flatten()
            .collect::<Vec<String>>()
            .join("\n")
    };

    let logo_col = logo.unwrap_or_else(|| "".into());

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
