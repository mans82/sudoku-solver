use std::fs::File;
use std::io::{BufRead, BufReader};

use super::sudoku::solver::SudokuSolver;
use super::sudoku::SudokuTable;

const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct App {
    config: AppConfig,
}

impl App {
    pub fn new(config: AppConfig) -> App {
        App { config }
    }

    pub fn run(&self) -> Result<(), String> {
        if self.config.print_version {
            println!("{} v{}", PACKAGE_NAME, VERSION);
            return Ok(());
        }

        if let None = self.config.file_name {
            return Err(String::from("Input file name not specified"));
        }

        let input_reader = Self::open_reader_to_file(self.config.file_name.as_ref().unwrap())?;
        let input_file = Self::read_input(input_reader);

        let input_table = SudokuTable::from_string(input_file?.into_iter())?;

        Self::print_solutions(&mut SudokuSolver::new(&input_table));

        Ok(())
    }

    fn open_reader_to_file(path: &str) -> Result<BufReader<File>, String> {
        let input_file = match File::open(path) {
            Ok(x) => x,
            Err(e) => {
                return Err(format!("Cannot open {}: {}", path, e.to_string()));
            }
        };

        Ok(BufReader::new(input_file))
    }

    fn read_input(reader: impl BufRead) -> Result<Vec<String>, String> {
        reader
            .lines()
            .map(|x| match x {
                Ok(x) => Ok(x),
                Err(e) => Err(format!("Error reading file: {}", e)),
            })
            .take(10)
            .collect()
    }

    fn print_solutions(solver: &mut SudokuSolver) {
        for (i, solution) in solver.enumerate() {
            println!(" => Solution {}:\n{}", i + 1, solution);
        }
    }
}

pub struct AppConfig {
    file_name: Option<String>,
    print_version: bool,
}

impl AppConfig {
    pub fn new(file_name: Option<String>, print_version: bool) -> AppConfig {
        AppConfig {
            file_name,
            print_version,
        }
    }
}
