use std::{env, process::exit};

const ASCII_LOGO: &str = r"
         _ _    __     _       _     
        (_| |  / _|   | |     | |    
 _______ _| |_| |_ ___| |_ ___| |__  
|_  / _ | | __|  _/ _ | __/ __| '_ \ 
 / |  __| | |_| ||  __| || (__| | | |
/___\___|_|\__|_| \___|\__\___|_| |_|
";
pub struct Ctx {
    pub args: Args,
    pub width: Option<usize>,
}

#[derive(Debug, Default)]
pub struct Args {
    pub no_logo: bool,
    pub logo_only: bool,
    pub minimal: bool,
    pub exclusive_args: i32,
}

impl Ctx {
    pub fn new() -> Self {
        let args = Args::parse_args();
        let width = termsize::get().map(|w| w.cols.into());
        Ctx { width, args }
    }
}

// TO-DO look into replacing this with clap sometime.
impl Args {
    pub fn parse_args() -> Self {
        let mut args = Args::default();
        let arg_vec: Vec<String> = env::args().collect();

        for arg in &arg_vec {
            match arg.as_str() {
                "--help" | "-h" => {
                    let help_text = format!(
                        "{}\n\
                  /////////////////////////////////////\n\n\
                  Usage:\n\
                  zeitfetch [options]\n\
                  \n\
                  Options:\n\
                  -h, --help      Show this help message\n\
                  -v, --version   Show the version number\n\
                  --no-logo       Display only system information\n\
                  --logo-only     Display only distro logo\n\
                  --minimal       Display logo and user prompt, vertically\n",
                        ASCII_LOGO
                    );
                    println!("{}", help_text);
                    exit(0);
                }
                "--version" | "-v" => {
                    let version_text = env!("CARGO_PKG_VERSION");
                    println!("zeitfetch v{}", version_text);
                    exit(0);
                }
                "--no-logo" => {
                    args.no_logo = true;
                    args.exclusive_args += 1;
                }
                "--logo-only" => {
                    args.logo_only = true;
                    args.exclusive_args += 1;
                }
                "--minimal" => {
                    args.minimal = true;
                }
                _ => {}
            }
        }

        if args.exclusive_args > 1 {
            println!("Please include only one of the following arguments:\n--no-logo, --logo-only, --minimal");
            exit(1);
        }

        args
    }
}
