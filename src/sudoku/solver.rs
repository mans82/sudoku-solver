use super::{CellLocation, SudokuCell, SudokuTable};

struct RecursionState {
    attempted_cell: CellLocation,
    possible_values: Vec<u8>,
}
pub struct SudokuSolver {
    table: SudokuTable,
    recursion_stack: Vec<RecursionState>,
}

impl SudokuSolver {
    pub fn new(table: &SudokuTable) -> SudokuSolver {
        let mut result = SudokuSolver {
            table: table.clone(),
            recursion_stack: Vec::with_capacity(81),
        };

        if let Some(cell) = result.next_empty_cell_starting_from(CellLocation { row: 0, col: 0 }) {
            let initial_state = RecursionState {
                attempted_cell: cell,
                possible_values: result.possible_values(cell),
            };
            result.recursion_stack.push(initial_state);
        }

        result
    }

    fn next_empty_cell_starting_from(
        &self,
        CellLocation { row: x, col: y }: CellLocation,
    ) -> Option<CellLocation> {
        for i in x..self.table.contents().len() {
            let starting_col = match i > x {
                true => 0,
                false => y,
            };
            for j in starting_col..self.table.contents().len() {
                if let SudokuCell::Empty = self.table.contents()[i][j] {
                    return Some(CellLocation { row: i, col: j });
                }
            }
        }

        None
    }

    fn possible_values(&self, cell: CellLocation) -> Vec<u8> {
        let mut existing_digits = [false; 9];

        self.mark_existing_row_values_in_array(cell.row, &mut existing_digits);
        self.mark_existing_col_values_in_array(cell.col, &mut existing_digits);
        self.mark_existing_values_spanning_3_by_3_cell_in_array(cell, &mut existing_digits);

        existing_digits
            .iter()
            .enumerate()
            .filter(|x| !x.1)
            .map(|x| x.0 as u8 + 1)
            .collect()
    }

    fn mark_existing_row_values_in_array(&self, row_index: usize, mark_array: &mut [bool; 9]) {
        for row_cell in &self.table.contents()[row_index] {
            if let SudokuCell::Filled(value) = row_cell {
                let value = *value as usize - 1;
                mark_array[value] = true;
            }
        }
    }

    fn mark_existing_col_values_in_array(&self, col_index: usize, mark_array: &mut [bool; 9]) {
        for row in self.table.contents() {
            let col_cell = row[col_index];
            if let SudokuCell::Filled(value) = col_cell {
                let value = value as usize - 1;
                mark_array[value] = true;
            }
        }
    }

    fn mark_existing_values_spanning_3_by_3_cell_in_array(
        &self,
        cell: CellLocation,
        mark_array: &mut [bool; 9],
    ) {
        for inside_cell in Self::cells_inside_3_by_3_cell(Self::index_of_3_by_3_cell(cell)) {
            if let SudokuCell::Filled(value) =
                self.table.contents()[inside_cell.row][inside_cell.col]
            {
                let value = value as usize - 1;
                mark_array[value] = true;
            }
        }
    }

    fn index_of_3_by_3_cell(cell: CellLocation) -> CellLocation {
        CellLocation {
            row: cell.row / 3,
            col: cell.col / 3,
        }
    }

    fn cells_inside_3_by_3_cell(the_3_by_3_cell: CellLocation) -> [CellLocation; 9] {
        let top_left_cell = CellLocation {
            row: the_3_by_3_cell.row * 3,
            col: the_3_by_3_cell.col * 3,
        };

        [
            CellLocation {
                row: top_left_cell.row,
                col: top_left_cell.col,
            },
            CellLocation {
                row: top_left_cell.row,
                col: top_left_cell.col + 1,
            },
            CellLocation {
                row: top_left_cell.row,
                col: top_left_cell.col + 2,
            },
            CellLocation {
                row: top_left_cell.row + 1,
                col: top_left_cell.col,
            },
            CellLocation {
                row: top_left_cell.row + 1,
                col: top_left_cell.col + 1,
            },
            CellLocation {
                row: top_left_cell.row + 1,
                col: top_left_cell.col + 2,
            },
            CellLocation {
                row: top_left_cell.row + 2,
                col: top_left_cell.col,
            },
            CellLocation {
                row: top_left_cell.row + 2,
                col: top_left_cell.col + 1,
            },
            CellLocation {
                row: top_left_cell.row + 2,
                col: top_left_cell.col + 2,
            },
        ]
    }

    fn try_next_possible_value(
        table: &mut SudokuTable,
        last_state: &mut RecursionState,
    ) -> Result<(), ()> {
        if let Some(next_value) = last_state.possible_values.last() {
            let CellLocation { row: x, col: y } = last_state.attempted_cell;
            table.contents_mut()[x][y] = SudokuCell::Filled(*next_value);
            last_state.possible_values.pop();

            Ok(())
        } else {
            Err(())
        }
    }

    fn presolve_next_empty_cell(&self, last_state: &RecursionState) -> Result<RecursionState, ()> {
        let CellLocation { row: x, col: y } = last_state.attempted_cell;
        let empty_cell = self.next_empty_cell_starting_from(CellLocation { row: x, col: y + 1 });
        if let None = empty_cell {
            Err(())
        } else {
            let empty_cell = empty_cell.unwrap();

            Ok(RecursionState {
                attempted_cell: empty_cell,
                possible_values: self.possible_values(empty_cell),
            })
        }
    }

    fn clear_last_try(recursion_stack: &mut Vec<RecursionState>, table: &mut SudokuTable) {
        let RecursionState {
            attempted_cell: CellLocation { row: x, col: y },
            ..
        } = recursion_stack.pop().unwrap();
        table.contents_mut()[x][y] = SudokuCell::Empty;
    }
}

impl Iterator for SudokuSolver {
    type Item = SudokuTable;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(last_state) = self.recursion_stack.last_mut() {
            if let Ok(_) = Self::try_next_possible_value(&mut self.table, last_state) {
                let last_state = self.recursion_stack.last().unwrap();
                if let Ok(presolved_state) = self.presolve_next_empty_cell(last_state) {
                    self.recursion_stack.push(presolved_state);
                } else {
                    return Some(self.table.clone());
                }
            } else {
                Self::clear_last_try(&mut self.recursion_stack, &mut self.table);
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
        let input_puzzle = "XX1XXXXX2\n\
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

        let mut table = SudokuTable::from_string(input_puzzle.lines()).unwrap();
        let mut solver = SudokuSolver::new(&mut table);

        let solution = solver.next().unwrap();

        assert_eq!(
            solution.contents,
            SudokuTable::from_string(solution_string.lines())
                .unwrap()
                .contents
        );

        assert!(solver.next().is_none());
    }
}
