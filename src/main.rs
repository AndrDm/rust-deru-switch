//! Tray program for Windows. The library crate decides; this file only
//! talks to the system. Typed text is never written anywhere.

#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

#[cfg(windows)]
mod win;

#[cfg(windows)]
fn main() {
    let force_key = match zyabra::parse_args(std::env::args().skip(1)) {
        Ok(zyabra::Launch::Help) => {
            explain(&zyabra::help_text(&zyabra::installed_path()), false);
            return;
        }
        Ok(zyabra::Launch::Run(key)) => key,
        Err(err) => {
            explain(&err, true);
            std::process::exit(1);
        }
    };
    if let Err(err) = win::run(force_key) {
        win::report_error(&err);
    }
}

#[cfg(not(windows))]
fn main() {
    eprintln!("Зябра работает только в Windows.");
    std::process::exit(1);
}

#[cfg(windows)]
fn explain(message: &str, error: bool) {
    if win::write_console(message, error) {
        return;
    }
    if error {
        win::report_error(message);
    } else {
        win::report_info(message);
    }
}
