use std::fs::File;
use std::io::{BufRead, BufReader};

use super::sudoku::solver::SudokuSolver;
use super::sudoku::SudokuTable;

pub struct App {
    config: AppConfig,
}

impl App {
    pub fn new(config: AppConfig) -> App {
        App { config }
    }

    pub fn run(&self) -> Result<(), String> {
        let input_reader = Self::open_reader_to_file(&self.config.filename)?;
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
    filename: String,
}

impl AppConfig {
    pub fn new(filename: &str) -> AppConfig {
        AppConfig {
            filename: String::from(filename),
        }
    }
}
