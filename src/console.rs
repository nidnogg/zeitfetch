// Windows doesn't always have ANSI color processing enabled by default.
// This function attempts to safely force-enable it.
pub fn try_prepare_colors() -> bool {
    #[cfg(windows)] {
        if cfg!(target_os = "windows") {
            use winapi::um::wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING;
            use winapi::um::winbase::STD_OUTPUT_HANDLE;
            use winapi::um::processenv::GetStdHandle;
            use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};

            unsafe {
                let console_handle = GetStdHandle(STD_OUTPUT_HANDLE);
                let mut current_mode: u32 = 0;

                if console_handle == std::ptr::null_mut() {
                    return false;
                }

                if 0 == GetConsoleMode(console_handle, &mut current_mode) {
                    return false;
                }

                if 0 == SetConsoleMode(console_handle, current_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING) {
                    return false;
                }
            }
        }
    }

    true
}