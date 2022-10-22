use std::env;
use std::process::exit;

use sudoku_solver::{App, AppConfig};

fn main() {
    let app_config = parse_args();
    let app = App::new(app_config);

    let result = app.run();

    if let Err(e) = result {
        exit_with_error_message(&e);
    }
}

fn exit_with_error_message(message: &str) -> ! {
    eprintln!(" !=> Error:\n\t{}", message);
    exit(1)
}

fn parse_args() -> AppConfig {
    let mut file_name: Option<String> = None;
    let mut print_version = false;

    for arg in env::args().skip(1) {
        if arg == "--version" {
            print_version = true;
            break;
        } else if file_name.is_none() {
            file_name = Some(arg);
        }
    }

    AppConfig::new(file_name, print_version)
}
