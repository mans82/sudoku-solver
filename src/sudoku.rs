use std::fmt::Display;

pub mod app;
pub mod solver;

#[derive(Clone, Copy)]
pub struct CellLocation {
    row: usize,
    col: usize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SudokuCell {
    Empty,
    Filled(u8),
}

impl Display for SudokuCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result_char = match self {
            Self::Empty => ' ',
            Self::Filled(x) if *x <= 9 => std::char::from_digit(*x as u32, 10).unwrap(),
            _ => '!',
        };

        write!(f, "{}", result_char)
    }
}

#[derive(Clone)]
pub struct SudokuTable {
    contents: Vec<Vec<SudokuCell>>,
}

impl SudokuTable {
    const TABLE_SIZE: usize = 9;

    pub fn from_string<T: Iterator<Item = String>>(table_str: T) -> Result<SudokuTable, String> {
        let contents: Result<Vec<Vec<SudokuCell>>, _> = table_str
            .map(Self::extract_row_from_line)
            .enumerate()
            .map(|(i, x)| match i >= Self::TABLE_SIZE {
                true => Err(String::from("Invalid input: expected 9 lines, found more")),
                false => x,
            })
            .collect();

        let result = SudokuTable {
            contents: contents?,
        };

        if result.contents.len() < Self::TABLE_SIZE {
            Err(String::from(format!(
                "Invalid input: expected 9 lines, found {}",
                result.contents.len()
            )))
        } else if !result.is_valid_sudoku() {
            Err(String::from("Invalid input: illegal table"))
        } else {
            Ok(result)
        }
    }

    fn extract_row_from_line(line: String) -> Result<Vec<SudokuCell>, String> {
        if line.len() != 9 {
            return Err(String::from(
                "Invalid input: line should have exactly 9 characters",
            ));
        }

        let mut result = Vec::with_capacity(Self::TABLE_SIZE);

        for char in line.chars() {
            let extracted_cell = match char {
                '1'..='9' => SudokuCell::Filled(char.to_digit(10).unwrap() as u8),
                'X' => SudokuCell::Empty,
                _ => return Err(format!("Invalid input: illegal character '{}'", char)),
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

    fn write_top_row(f: &mut std::fmt::Formatter<'_>, values: &[SudokuCell]) -> std::fmt::Result {
        writeln!(f, "┌───┬───┬───┐ ┌───┬───┬───┐ ┌───┬───┬───┐")?;
        Self::write_middle_row(f, values)
    }

    fn write_middle_row(
        f: &mut std::fmt::Formatter<'_>,
        values: &[SudokuCell],
    ) -> std::fmt::Result {
        Self::write_row_of_nums(f, values)?;
        writeln!(f, "├───┼───┼───┤ ├───┼───┼───┤ ├───┼───┼───┤")
    }

    fn write_row_of_nums(
        f: &mut std::fmt::Formatter<'_>,
        values: &[SudokuCell],
    ) -> std::fmt::Result {
        writeln!(
            f,
            "│ {} │ {} │ {} │ │ {} │ {} │ {} │ │ {} │ {} │ {} │",
            values[0],
            values[1],
            values[2],
            values[3],
            values[4],
            values[5],
            values[6],
            values[7],
            values[8]
        )
    }

    fn write_bottom_row(
        f: &mut std::fmt::Formatter<'_>,
        values: &[SudokuCell],
    ) -> std::fmt::Result {
        Self::write_row_of_nums(f, values)?;
        writeln!(f, "└───┴───┴───┘ └───┴───┴───┘ └───┴───┴───┘")
    }
}

impl Display for SudokuTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0usize..3 {
            let row_start = 3 * i;
            Self::write_top_row(f, &self.contents[row_start])?;
            Self::write_middle_row(f, &self.contents[row_start + 1])?;
            Self::write_bottom_row(f, &self.contents[row_start + 2])?;
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
            SudokuTable::from_string(correct_table_string.lines().map(String::from)).unwrap();

        assert_eq!(table.len(), 9);
        for row in &table {
            assert_eq!(row.len(), 9);
        }
    }

    #[test]
    #[should_panic(expected = "found 8")]
    fn incorrect_table_string_too_few_lines() {
        let incorrect_table_string = "3X65X84XX\n\
        52XXXXXXX\n\
        X87XXXX31\n\
        XX3X1XX8X\n\
        9XX863XX5\n\
        X5XX9X6XX\n\
        13XXXX25X\n\
        XX52X63XX\n";

        SudokuTable::from_string(incorrect_table_string.lines().map(String::from)).unwrap();
    }

    #[test]
    #[should_panic(expected = "found more")]
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

        SudokuTable::from_string(incorrect_table_string.lines().map(String::from)).unwrap();
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

        SudokuTable::from_string(incorrect_table_string.lines().map(String::from)).unwrap();
    }

    #[test]
    #[should_panic(expected = "illegal table")]
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

        SudokuTable::from_string(incorrect_table_string.lines().map(String::from)).unwrap();
    }

    #[test]
    fn display() {
        let input_table = "391867542\n\
        286534719\n\
        457291386\n\
        129645873\n\
        638179254\n\
        745328691\n\
        972486135\n\
        564713928\n\
        813952467\n";

        let correct_display = "┌───┬───┬───┐ ┌───┬───┬───┐ ┌───┬───┬───┐\n\
        │ 3 │ 9 │ 1 │ │ 8 │ 6 │ 7 │ │ 5 │ 4 │ 2 │\n\
        ├───┼───┼───┤ ├───┼───┼───┤ ├───┼───┼───┤\n\
        │ 2 │ 8 │ 6 │ │ 5 │ 3 │ 4 │ │ 7 │ 1 │ 9 │\n\
        ├───┼───┼───┤ ├───┼───┼───┤ ├───┼───┼───┤\n\
        │ 4 │ 5 │ 7 │ │ 2 │ 9 │ 1 │ │ 3 │ 8 │ 6 │\n\
        └───┴───┴───┘ └───┴───┴───┘ └───┴───┴───┘\n\
        ┌───┬───┬───┐ ┌───┬───┬───┐ ┌───┬───┬───┐\n\
        │ 1 │ 2 │ 9 │ │ 6 │ 4 │ 5 │ │ 8 │ 7 │ 3 │\n\
        ├───┼───┼───┤ ├───┼───┼───┤ ├───┼───┼───┤\n\
        │ 6 │ 3 │ 8 │ │ 1 │ 7 │ 9 │ │ 2 │ 5 │ 4 │\n\
        ├───┼───┼───┤ ├───┼───┼───┤ ├───┼───┼───┤\n\
        │ 7 │ 4 │ 5 │ │ 3 │ 2 │ 8 │ │ 6 │ 9 │ 1 │\n\
        └───┴───┴───┘ └───┴───┴───┘ └───┴───┴───┘\n\
        ┌───┬───┬───┐ ┌───┬───┬───┐ ┌───┬───┬───┐\n\
        │ 9 │ 7 │ 2 │ │ 4 │ 8 │ 6 │ │ 1 │ 3 │ 5 │\n\
        ├───┼───┼───┤ ├───┼───┼───┤ ├───┼───┼───┤\n\
        │ 5 │ 6 │ 4 │ │ 7 │ 1 │ 3 │ │ 9 │ 2 │ 8 │\n\
        ├───┼───┼───┤ ├───┼───┼───┤ ├───┼───┼───┤\n\
        │ 8 │ 1 │ 3 │ │ 9 │ 5 │ 2 │ │ 4 │ 6 │ 7 │\n\
        └───┴───┴───┘ └───┴───┴───┘ └───┴───┴───┘\n";

        let sudoku_table = SudokuTable::from_string(input_table.lines().map(String::from)).unwrap();

        assert_eq!(format!("{}", sudoku_table).trim(), correct_display.trim());
    }
}
