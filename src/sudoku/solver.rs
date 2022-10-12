use super::{SudokuCell, SudokuTable};

type TableCoordinates = (usize, usize);
type RecursionState = (TableCoordinates, Vec<u8>);
pub struct SudokuSolver<'a> {
    table: &'a mut SudokuTable,
    recursion_stack: Vec<RecursionState>,
}

impl<'a> SudokuSolver<'a> {
    pub fn new(table: &'a mut SudokuTable) -> SudokuSolver<'a> {
        let mut result = SudokuSolver {
            table,
            recursion_stack: Vec::with_capacity(81),
        };

        if let Some((i, j)) = result.next_empty_cell((0, 0)) {
            result
                .recursion_stack
                .push(((i, j), result.possible_values((i, j))));
        }

        result
    }

    fn next_empty_cell(&self, (x, y): TableCoordinates) -> Option<TableCoordinates> {
        for i in x..self.table.table().len() {
            let starting_col = match i > x {
                true => 0,
                false => y,
            };
            for j in starting_col..self.table.table().len() {
                if let SudokuCell::Empty = self.table.table()[i][j] {
                    return Some((i, j));
                }
            }
        }

        None
    }

    fn possible_values(&self, (i, j): TableCoordinates) -> Vec<u8> {
        let mut existing_digits = [false; 9];

        for row_value in &self.table.table()[i] {
            if let SudokuCell::Filled(x) = row_value {
                let x = *x as usize - 1;
                existing_digits[x] = true;
            }
        }

        for row in self.table.table() {
            let col_value = row[j];
            if let SudokuCell::Filled(x) = col_value {
                let x = x as usize - 1;
                existing_digits[x] = true;
            }
        }

        for indices_of_3_by_3_cell in
            Self::all_indices_of_3_by_3_cell(Self::index_of_3_by_3_cell((i, j)))
        {
            let (i, j) = indices_of_3_by_3_cell;
            if let SudokuCell::Filled(x) = self.table.table()[i][j] {
                let x = x as usize - 1;
                existing_digits[x] = true;
            }
        }

        existing_digits
            .iter()
            .enumerate()
            .filter(|x| !x.1)
            .map(|x| x.0 as u8 + 1)
            .collect()
    }

    fn index_of_3_by_3_cell((i, j): TableCoordinates) -> TableCoordinates {
        (i / 3, j / 3)
    }

    fn all_indices_of_3_by_3_cell((i, j): TableCoordinates) -> [TableCoordinates; 9] {
        let top_left_cell_index = (i * 3, j * 3);

        [
            (top_left_cell_index.0, top_left_cell_index.1),
            (top_left_cell_index.0, top_left_cell_index.1 + 1),
            (top_left_cell_index.0, top_left_cell_index.1 + 2),
            (top_left_cell_index.0 + 1, top_left_cell_index.1),
            (top_left_cell_index.0 + 1, top_left_cell_index.1 + 1),
            (top_left_cell_index.0 + 1, top_left_cell_index.1 + 2),
            (top_left_cell_index.0 + 2, top_left_cell_index.1),
            (top_left_cell_index.0 + 2, top_left_cell_index.1 + 1),
            (top_left_cell_index.0 + 2, top_left_cell_index.1 + 2),
        ]
    }
}

impl Iterator for SudokuSolver<'_> {
    type Item = SudokuTable;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(recursion_state) = self.recursion_stack.last_mut() {
            if let Some(new_value) = recursion_state.1.pop() {
                let (i, j) = recursion_state.0;
                self.table.table_mut()[i][j] = SudokuCell::Filled(new_value);

                let next_empty_cell = self.next_empty_cell((i, j + 1));

                if let Some(empty_cell_coordinates) = next_empty_cell {
                    self.recursion_stack.push((
                        empty_cell_coordinates,
                        self.possible_values(empty_cell_coordinates),
                    ));
                } else {
                    return Some(self.table.clone());
                }
            } else {
                let ((i, j), ..) = self.recursion_stack.pop().unwrap();
                self.table.table_mut()[i][j] = SudokuCell::Empty;
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::sudoku::SudokuTable;

    use super::SudokuSolver;

    #[test]
    fn single_solution() {
        let input_table = "XX1XXXXX2\n\
        XXXX34XXX\n\
        X5XXX1XX6\n\
        X2X6XXXX3\n\
        X3XXXXX5X\n\
        7XXXX8X9X\n\
        9XX4XXX3X\n\
        XXX71XXXX\n\
        8XXXXX4XX";

        let solution_string = "391867542\n\
        286534719\n\
        457291386\n\
        129645873\n\
        638179254\n\
        745328691\n\
        972486135\n\
        564713928\n\
        813952467\n";

        let mut table = SudokuTable::from_string(input_table).unwrap();
        let mut solver = SudokuSolver::new(&mut table);

        let solution = solver.next().unwrap();

        assert_eq!(
            solution.table,
            SudokuTable::from_string(solution_string).unwrap().table
        );

        assert!(solver.next().is_none());
    }
}
