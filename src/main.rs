use std::env;
use std::process::exit;

use sudoku_solver::sudoku::app::{App, AppConfig};

fn main() {
    let filename = env::args().skip(1).next();

    if filename.is_none() {
        exit_with_error_message("Input file name not specified in arguments");
    }

    let app_config = AppConfig::new(&filename.unwrap());
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
