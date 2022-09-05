pub fn get_logo_by_distro(sys_name: &str) -> String {

    match sys_name {
        "win11" => {
            let logo = String::from("
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll  
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll 
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll
                                
\x1b[34;1mlllllllllllllll   lllllllllllllll 
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll\x1b[0m
    ");
        logo
        },
        "win" => {
            let logo = String::from("
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll  
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll 
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll
                                
\x1b[34;1mlllllllllllllll   lllllllllllllll 
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll\x1b[0m
    ");
        logo
        },
        _ => String::from(""),
    }
}