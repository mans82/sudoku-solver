#[derive(Clone, Copy)]
pub enum SudokuCell {
    Empty,
    Filled(u8),
}

pub struct SudokuTable {
    table: Vec<Vec<SudokuCell>>,
    filled_cells: u32,
}

impl SudokuTable {
    pub fn from_string(table_str: &str) -> Result<SudokuTable, String> {
        let mut table: Vec<Vec<SudokuCell>> = vec![vec![SudokuCell::Empty; 9]; 9];
        let mut filled_cells: u32 = 0;

        let mut max_reached_row: usize = 0;

        for (row, line) in table_str.lines().enumerate() {
            max_reached_row = row;

            if row >= 9 {
                return Err(String::from("Malformed string: too many lines"));
            }

            if line.len() != 9 {
                return Err(String::from(
                    "Malformed line: line should have exactly 9 characters",
                ));
            }

            for (col, char) in line.chars().enumerate() {
                table[row][col] = match char {
                    '1'..='9' => {
                        filled_cells += 1;
                        SudokuCell::Filled(char.to_digit(10).unwrap() as u8)
                    }
                    'X' => SudokuCell::Empty,
                    _ => return Err(format!("Illegal character: {}", char)),
                }
            }
        }

        if max_reached_row < 8 {
            Err(String::from("Malformed string: too few lines"))
        } else if !Self::is_valid_sudoku(&table) {
            Err(String::from("Input sudoku table is invalid"))
        } else {
            Ok(SudokuTable {
                table,
                filled_cells,
            })
        }
    }

    fn is_valid_sudoku(table_cells: &Vec<Vec<SudokuCell>>) -> bool {
        Self::are_rows_valid(table_cells)
            && Self::are_cols_valid(table_cells)
            && Self::are_3_by_3_cells_valid(table_cells)
    }

    fn are_rows_valid(table_cells: &Vec<Vec<SudokuCell>>) -> bool {
        for i in 0usize..9 {
            let mut row_digits = vec![];
            for j in 0usize..9 {
                if let SudokuCell::Filled(x) = table_cells[i][j] {
                    row_digits.push(x);
                }
            }

            if !Self::are_distinct_digits(&row_digits) {
                return false;
            }
        }

        true
    }

    fn are_cols_valid(table_cells: &Vec<Vec<SudokuCell>>) -> bool {
        for j in 0usize..9 {
            let mut col_digits = vec![];

            for i in 0usize..9 {
                if let SudokuCell::Filled(x) = table_cells[i][j] {
                    col_digits.push(x);
                }
            }

            if !Self::are_distinct_digits(&col_digits) {
                return false;
            }
        }

        true
    }

    fn are_3_by_3_cells_valid(table_cells: &Vec<Vec<SudokuCell>>) -> bool {
        for i in 0usize..3 {
            for j in 0usize..3 {
                if !Self::are_distinct_digits(&Self::get_3_by_3_cell(table_cells, i, j)) {
                    return false;
                }
            }
        }

        true
    }

    fn get_3_by_3_cell(table_cells: &Vec<Vec<SudokuCell>>, row: usize, col: usize) -> Vec<u8> {
        let mut result = vec![];

        for i in 0usize..3 {
            for j in 0usize..3 {
                let table_i = (3 * row) + i;
                let table_j = (3 * col) + j;

                if let SudokuCell::Filled(x) = table_cells[table_i][table_j] {
                    result.push(x);
                }
            }
        }

        result
    }

    fn are_distinct_digits(digits: &[u8]) -> bool {
        let digit_exists: &mut [bool] = &mut [false; 9];

        for digit in digits {
            let digit = *digit as usize - 1;

            if digit_exists[digit] {
                return false;
            } else {
                digit_exists[digit] = true;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::SudokuTable;

    #[test]
    fn correct_table_string() {
        let correct_table_string = "3X65X84XX\n\
        52XXXXXXX\n\
        X87XXXX31\n\
        XX3X1XX8X\n\
        9XX863XX5\n\
        X5XX9X6XX\n\
        13XXXX25X\n\
        XXXXXXX74\n\
        XX52X63XX\n";

        let SudokuTable {
            table,
            filled_cells,
        } = SudokuTable::from_string(correct_table_string).unwrap();

        assert_eq!(table.len(), 9);
        for row in &table {
            assert_eq!(row.len(), 9);
        }
        assert_eq!(filled_cells, 81 - 49);
    }

    #[test]
    #[should_panic(expected = "too few lines")]
    fn incorrect_table_string_too_few_lines() {
        let incorrect_table_string = "3X65X84XX\n\
        52XXXXXXX\n\
        X87XXXX31\n\
        XX3X1XX8X\n\
        9XX863XX5\n\
        X5XX9X6XX\n\
        13XXXX25X\n\
        XX52X63XX\n";

        SudokuTable::from_string(incorrect_table_string).unwrap();
    }

    #[test]
    #[should_panic(expected = "too many lines")]
    fn incorrect_table_string_too_many_lines() {
        let incorrect_table_string = "3X65X84XX\n\
        52XXXXXXX\n\
        X87XXXX31\n\
        XX3X1XX8X\n\
        9XX863XX5\n\
        X5XX9X6XX\n\
        13XXXX25X\n\
        XXXXXXX74\n\
        XX52X63XX\n
        7XXXXXXXX\n";

        SudokuTable::from_string(incorrect_table_string).unwrap();
    }

    #[test]
    #[should_panic(expected = "have exactly 9 characters")]
    fn incorrect_table_string_invalid_line_length() {
        let incorrect_table_string = "3X65X84XX\n\
        52XXXXXXX\n\
        X87XXXX31\n\
        XX3X1XX8X\n\
        9XX863XX5\n\
        X5XX9X6XX\n\
        13XXXX25X\n\
        XXXXXXX7\n\
        XX52X63XX\n";

        SudokuTable::from_string(incorrect_table_string).unwrap();
    }

    #[test]
    #[should_panic(expected = "sudoku table is invalid")]
    fn incorrect_table_string_invalid_col() {
        let incorrect_table_string = "3X65X84XX\n\
        52XXXXXXX\n\
        X87XXXX31\n\
        XX3X1XX8X\n\
        9XX863XX5\n\
        X5XX9X6XX\n\
        13XXXX25X\n\
        XXXXXXX74\n\
        5X52X63XX\n";

        SudokuTable::from_string(incorrect_table_string).unwrap();
    }
}
