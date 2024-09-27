use std::ops::Range;
use std::usize;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use crate::enums::sudoku_difficulty::SudokuDifficulty;
use crate::constants::*;
use crate::types::*;

pub struct Sudoku
{
    grid: SudokuGrid,
    solved: bool
}

impl Sudoku {
    pub fn new(grid: Option<SudokuGrid>) -> Self
    {
        return match grid {
            None => Self {
                grid: [[0u8; GRID_SIZE]; GRID_SIZE],
                solved: false
            },
            Some(grid) => Self {
                grid,
                solved: false
            }
        };
    }

    pub fn print(&self) -> ()
    {
        for row in &self.grid {
            for cell in row {
                print!("{} ", cell);
            }

            println!();
        }
    }

    fn find_empty_cells(&self) -> Vec<SudokuCellIndex>
    {
        let mut empty_cells: Vec<SudokuCellIndex> = vec![];

        for row_index in INDEX_RANGE {
            for column_index in INDEX_RANGE {
                if self.grid[row_index][column_index] == 0 {
                    empty_cells.push([row_index, column_index]);
                }
            }
        }

        return empty_cells;
    }

    fn is_valid_placement(&self, row: usize, column: usize, number: u8) -> bool
    {
        for row_index in INDEX_RANGE {
            if self.grid[row_index][column] == number {
                return false;
            }
        }

        for column_index in INDEX_RANGE {
            if self.grid[row][column_index] == number {
                return false;
            }
        }

        let subgrid_row: usize = (row / SUBGRID_SIZE) * SUBGRID_SIZE;
        let subgrid_column: usize = (column / SUBGRID_SIZE) * SUBGRID_SIZE;

        for subgrid_row_index in 0..SUBGRID_SIZE {
            for subgrid_column_index in 0..SUBGRID_SIZE {
                if self.grid[subgrid_row + subgrid_row_index][subgrid_column + subgrid_column_index] == number {
                    return false;
                }
            }
        }

        return true;
    }

    fn generate_shuffled_array(&self) -> SudokuRow
    {
        let mut numbers: SudokuRow = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        
        numbers.shuffle(&mut thread_rng());

        return numbers;
    }

    fn solve_recursively(&mut self, empty_cells: &[SudokuCellIndex], nth_empty_cell: usize) -> bool
    {
        if nth_empty_cell >= empty_cells.len() {
            return true;
        }

        let [row_index, column_index]: SudokuCellIndex = empty_cells[nth_empty_cell];
        let numbers: SudokuRow = self.generate_shuffled_array();

        for number in numbers {
            if self.is_valid_placement(row_index, column_index, number) {
                self.grid[row_index][column_index] = number;

                if self.solve_recursively(empty_cells, nth_empty_cell + 1) {
                    return true;
                }

                self.grid[row_index][column_index] = 0;
            }
        }

        return false;
    }

    fn solve(&mut self) -> ()
    {
        let empty_cells: Vec<SudokuCellIndex> = self.find_empty_cells();

        self.solve_recursively(&empty_cells, 0);

        self.solved = true;
    }

    fn count_solutions_recursively(&mut self, empty_cells: &[SudokuCellIndex], nth_empty_cell: usize, number_of_solutions: &mut u8) -> ()
    {
        if nth_empty_cell >= empty_cells.len() {
            *number_of_solutions += 1;
            return;
        }

        let [row_index, column_index]: SudokuCellIndex = empty_cells[nth_empty_cell];
        let numbers: SudokuRow = self.generate_shuffled_array();

        for number in numbers {
            if *number_of_solutions > 1 {
                return;
            }

            if self.is_valid_placement(row_index, column_index, number) {
                self.grid[row_index][column_index] = number;
                self.count_solutions_recursively(empty_cells, nth_empty_cell + 1, number_of_solutions);
                self.grid[row_index][column_index] = 0;
            }
        }
    }

    fn count_solutions(&mut self, number_of_solutions: &mut u8) -> ()
    {
        let empty_cells: Vec<SudokuCellIndex> = self.find_empty_cells();

        self.count_solutions_recursively(&empty_cells, 0, number_of_solutions);
    }

    fn has_unique_solution(&mut self) -> bool
    {
        let mut number_of_solutions: u8 = 0;

        self.count_solutions(&mut number_of_solutions);

        return number_of_solutions == 1;
    }

    fn remove_some_cells(&mut self, number_of_cells_to_remove: u8) -> ()
    {
        let mut rng = thread_rng();
        let mut cells_removed: u8 = 0;

        while cells_removed < number_of_cells_to_remove {
            let random_row_index: usize = rng.gen_range(INDEX_RANGE);
            let random_column_index: usize = rng.gen_range(INDEX_RANGE);

            if self.grid[random_row_index][random_column_index] != 0 {
                let backup_number: u8 = self.grid[random_row_index][random_column_index];
                self.grid[random_row_index][random_column_index] = 0;

                if self.has_unique_solution() {
                    cells_removed += 1;
                } else {
                    self.grid[random_row_index][random_column_index] = backup_number;
                }
            }
        }

        self.solved = false;
    }

    pub fn generate(&mut self, difficulty: SudokuDifficulty) -> &mut Self
    {
        self.grid[0] = self.generate_shuffled_array();

        self.solve();

        let range_of_cells_to_remove: Range<u8> = match difficulty {
            SudokuDifficulty::ChildPlay => 40..45,
            SudokuDifficulty::Easy => 45..50,
            SudokuDifficulty::Medium => 50..55,
            SudokuDifficulty::Hard => 55..60,
            SudokuDifficulty::Expert => 60..65,
        };
        let number_of_cells_to_remove: u8 = thread_rng().gen_range(range_of_cells_to_remove);

        self.remove_some_cells(number_of_cells_to_remove);

        return self;
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_creating_new_empty_grid()
    {
        let sudoku: Sudoku = Sudoku::new(None);

        assert_eq!(sudoku.grid, [[0u8; GRID_SIZE]; GRID_SIZE]);
    }

    #[test]
    fn test_creating_provided_grid()
    {
        let provided_grid: SudokuGrid = [
            [0, 7, 0, 0, 6, 3, 1, 8, 0],
            [0, 0, 8, 0, 0, 0, 0, 0, 0],
            [2, 0, 0, 0, 0, 1, 0, 0, 0],
            [0, 1, 4, 0, 0, 2, 0, 0, 6],
            [8, 5, 3, 0, 0, 0, 0, 7, 1],
            [0, 9, 0, 0, 0, 0, 0, 0, 4],
            [9, 0, 0, 0, 0, 0, 0, 6, 0],
            [0, 0, 0, 9, 0, 0, 0, 3, 8],
            [0, 2, 0, 7, 8, 0, 0, 1, 5]
        ];

        let sudoku: Sudoku = Sudoku::new(Some(provided_grid));
        assert_eq!(sudoku.grid, provided_grid);
    }

    #[test]
    fn test_find_empty_cells()
    {
        let mut sudoku: Sudoku = Sudoku::new(None);

        sudoku.grid[0][5] = 1;
        sudoku.grid[4][7] = 3;
        sudoku.grid[5][2] = 9;
        sudoku.grid[8][6] = 7;

        let empty_cells: Vec<SudokuCellIndex> = sudoku.find_empty_cells();

        assert_eq!(empty_cells.len(), 77);
    }

    #[test]
    fn test_is_valid_placement()
    {
        let mut sudoku: Sudoku = Sudoku::new(None);

        sudoku.grid[4][4] = 5;

        // Valid placement on row
        assert!(sudoku.is_valid_placement(4, 7, 1));
        assert!(!sudoku.is_valid_placement(4, 7, 5));

        // Valid placement on column
        assert!(sudoku.is_valid_placement(5, 4, 1));
        assert!(!sudoku.is_valid_placement(5, 4, 5));

        // Valid placement on subgrid
        assert!(sudoku.is_valid_placement(5, 5, 1));
        assert!(!sudoku.is_valid_placement(5, 5, 5));
    }

    #[test]
    fn test_has_unique_solution()
    {
        let mut sudoku_with_unique_solution: Sudoku = Sudoku::new(Some([
            [0, 7, 0, 0, 6, 3, 1, 8, 0],
            [0, 0, 8, 0, 0, 0, 0, 0, 0],
            [2, 0, 0, 0, 0, 1, 0, 0, 0],
            [0, 1, 4, 0, 0, 2, 0, 0, 6],
            [8, 5, 3, 0, 0, 0, 0, 7, 1],
            [0, 9, 0, 0, 0, 0, 0, 0, 4],
            [9, 0, 0, 0, 0, 0, 0, 6, 0],
            [0, 0, 0, 9, 0, 0, 0, 3, 8],
            [0, 2, 0, 7, 8, 0, 0, 1, 5]
        ]));

        let mut sudoku_with_multiple_solutions: Sudoku = Sudoku::new(Some([
            [2, 9, 5, 7, 4, 3, 8, 6, 1],
            [4, 3, 1, 8, 6, 5, 9, 0, 0],
            [8, 7, 6, 1, 9, 2, 5, 4, 3],
            [3, 8, 7, 4, 5, 9, 2, 1, 6],
            [6, 1, 2, 3, 8, 7, 4, 9, 5],
            [5, 4, 9, 2, 1, 6, 7, 3, 8],
            [7, 6, 3, 5, 3, 4, 1, 8, 9],
            [9, 2, 8, 6, 7, 1, 3, 5, 4],
            [1, 5, 4, 9, 3, 8, 6, 0, 0]
        ]));

        assert!(sudoku_with_unique_solution.has_unique_solution());
        assert!(!sudoku_with_multiple_solutions.has_unique_solution());
    }

    #[test]
    fn test_fill_grid()
    {
        let mut sudoku: Sudoku = Sudoku::new(None);

        sudoku.grid[0] = [0, 1, 4, 0, 0, 2, 0, 0, 6];

        sudoku.solve();

        assert!(sudoku.has_unique_solution());
    }

    #[test]
    fn test_remove_some_cells()
    {
        let mut sudoku: Sudoku = Sudoku::new(None);

        sudoku.solve();
        sudoku.remove_some_cells(10);

        assert_eq!(sudoku.find_empty_cells().len(), 10);
    }

    #[test]
    fn test_generate()
    {
        let mut sudoku: Sudoku = Sudoku::new(None);

        sudoku.generate(SudokuDifficulty::Hard);

        assert!(sudoku.has_unique_solution());
    }
}
