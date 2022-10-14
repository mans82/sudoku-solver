use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct CellLocation {
    row: usize,
    col: usize,
}

pub mod solver;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SudokuCell {
    Empty,
    Filled(u8),
}

#[derive(Clone)]
pub struct SudokuTable {
    contents: Vec<Vec<SudokuCell>>,
}

impl SudokuTable {
    const TABLE_SIZE: usize = 9;

    pub fn from_string(table_str: &str) -> Result<SudokuTable, String> {
        let contents: Result<Vec<Vec<SudokuCell>>, _> = table_str
            .lines()
            .map(Self::extract_row_from_line)
            .enumerate()
            .map(|(i, x)| match i >= Self::TABLE_SIZE {
                true => Err(String::from("Malformed string: too many lines")),
                false => x,
            })
            .collect();

        let result = SudokuTable {
            contents: contents?,
        };

        if result.contents.len() < Self::TABLE_SIZE {
            Err(String::from("Malformed string: too few lines"))
        } else if !result.is_valid_sudoku() {
            Err(String::from("Input sudoku table is invalid"))
        } else {
            Ok(result)
        }
    }

    fn extract_row_from_line(line: &str) -> Result<Vec<SudokuCell>, String> {
        if line.len() != 9 {
            return Err(String::from(
                "Malformed line: line should have exactly 9 characters",
            ));
        }

        let mut result = Vec::with_capacity(Self::TABLE_SIZE);

        for char in line.chars() {
            let extracted_cell = match char {
                '1'..='9' => SudokuCell::Filled(char.to_digit(10).unwrap() as u8),
                'X' => SudokuCell::Empty,
                _ => return Err(format!("Illegal character: {}", char)),
            };

            result.push(extracted_cell);
        }

        Ok(result)
    }

    fn is_valid_sudoku(&self) -> bool {
        self.are_rows_valid() && self.are_cols_valid() && self.are_3_by_3_cells_valid()
    }

    fn are_rows_valid(&self) -> bool {
        for i in 0usize..Self::TABLE_SIZE {
            let mut row_digits = vec![];
            for j in 0usize..Self::TABLE_SIZE {
                if let SudokuCell::Filled(x) = self.contents[i][j] {
                    row_digits.push(x);
                }
            }

            if !Self::are_distinct_digits(&row_digits) {
                return false;
            }
        }

        true
    }

    fn are_cols_valid(&self) -> bool {
        for j in 0usize..Self::TABLE_SIZE {
            let mut col_digits = vec![];

            for i in 0usize..Self::TABLE_SIZE {
                if let SudokuCell::Filled(x) = self.contents[i][j] {
                    col_digits.push(x);
                }
            }

            if !Self::are_distinct_digits(&col_digits) {
                return false;
            }
        }

        true
    }

    fn are_3_by_3_cells_valid(&self) -> bool {
        for i in 0usize..3 {
            for j in 0usize..3 {
                if !Self::are_distinct_digits(&self.get_3_by_3_cell(i, j)) {
                    return false;
                }
            }
        }

        true
    }

    fn get_3_by_3_cell(&self, row: usize, col: usize) -> Vec<u8> {
        let mut result = vec![];

        for i in 0usize..3 {
            for j in 0usize..3 {
                let table_i = (3 * row) + i;
                let table_j = (3 * col) + j;

                if let SudokuCell::Filled(x) = self.contents[table_i][table_j] {
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

    pub fn contents(&self) -> &Vec<Vec<SudokuCell>> {
        &self.contents
    }

    pub fn contents_mut(&mut self) -> &mut Vec<Vec<SudokuCell>> {
        &mut self.contents
    }
}

impl Display for SudokuTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.contents {
            writeln!(
                f,
                "{}",
                row.iter()
                    .map(|x| match x {
                        SudokuCell::Filled(x) => std::char::from_digit(*x as u32, 10).unwrap(),
                        SudokuCell::Empty => 'X',
                    })
                    .collect::<String>()
            )?;
        }

        Ok(())
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

        let SudokuTable { contents: table } =
            SudokuTable::from_string(correct_table_string).unwrap();

        assert_eq!(table.len(), 9);
        for row in &table {
            assert_eq!(row.len(), 9);
        }
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
