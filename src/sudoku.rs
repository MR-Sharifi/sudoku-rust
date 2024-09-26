use std::ops::Range;
use std::usize;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use crate::enums::sudoku_difficulty::SudokuDifficulty;
use crate::constants::*;
use crate::types::*;

pub struct Sudoku
{
    grid: SudokuGrid
}

impl Sudoku {
    pub fn new(grid: Option<SudokuGrid>) -> Self
    {
        return match grid {
            None => Self { grid: [[0u8; GRID_SIZE]; GRID_SIZE] },
            Some(grid) => Self { grid }
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

    fn find_empty_cells(&self) -> Vec<SudokuCell>
    {
        let mut empty_cells: Vec<SudokuCell> = vec![];

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

    fn fill_recursively(&mut self, empty_cells: &[SudokuCell], nth_empty_cell: usize) -> bool
    {
        if nth_empty_cell >= empty_cells.len() {
            return true;
        }

        let [row_index, column_index]: SudokuCell = empty_cells[nth_empty_cell];
        let numbers: SudokuRow = self.generate_shuffled_array();

        for number in numbers {
            if self.is_valid_placement(row_index, column_index, number) {
                self.grid[row_index][column_index] = number;

                if self.fill_recursively(empty_cells, nth_empty_cell + 1) {
                    return true;
                }

                self.grid[row_index][column_index] = 0;
            }
        }

        return false;
    }

    fn fill_grid(&mut self) -> ()
    {
        let empty_cells: Vec<SudokuCell> = self.find_empty_cells();

        self.fill_recursively(&empty_cells, 0);
    }

    fn count_solutions_recursively(&mut self, empty_cells: &[SudokuCell], nth_empty_cell: usize, number_of_solutions: &mut u8) -> ()
    {
        if nth_empty_cell >= empty_cells.len() {
            *number_of_solutions += 1;
            return;
        }

        let [row_index, column_index]: SudokuCell = empty_cells[nth_empty_cell];
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
        let empty_cells: Vec<SudokuCell> = self.find_empty_cells();

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
    }

    pub fn generate(&mut self, difficulty: SudokuDifficulty) -> &mut Self
    {
        self.grid[0] = self.generate_shuffled_array();

        self.fill_grid();

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