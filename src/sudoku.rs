use std::usize;
use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::enums::sudoku_difficulty::SudokuDifficulty;

pub struct Sudoku
{
    grid: [[u8; 9]; 9]
}

impl Sudoku {
    pub fn new(grid: Option<[[u8; 9]; 9]>) -> Self
    {
        return match grid {
            None => Self { grid: [[0u8; 9]; 9] },
            Some(grid) => Self { grid }
        };
    }

    pub fn print(self: &Self) -> ()
    {
        for row_index in 0..9 as usize {
            for column_index in 0..9 as usize {
                print!("{} ", self.grid[row_index][column_index]);
            }

            print!("\n");
        }
    }

    fn find_empty_cells(self: &Self) -> Vec<[usize; 2]>
    {
        let mut empty_cells: Vec<[usize; 2]> = vec![];

        for row_index in 0..9 as usize {
            for column_index in 0..9 as usize {
                if self.grid[row_index][column_index] == 0 {
                    empty_cells.push([row_index, column_index]);
                }
            }
        }

        return empty_cells;
    }

    fn is_valid_placement(self: &Self, row: usize, column: usize, number: u8) -> bool
    {
        for row_index in 0..9 as usize {
            if self.grid[row_index][column] == number {
                return false;
            }
        }

        for column_index in 0..9 as usize {
            if self.grid[row][column_index] == number {
                return false;
            }
        }

        let subgrid_row: usize = (row / 3) * 3;
        let subgrid_column: usize = (column / 3) * 3;

        for subgrid_row_index in 0..3 as usize {
            for subgrid_column_index in 0..3 as usize {
                if self.grid[subgrid_row + subgrid_row_index][subgrid_column + subgrid_column_index] == number {
                    return false;
                }
            }
        }

        return true;
    }

    fn generate_shuffled_vector(self: &Self) -> Vec<u8>
    {
        let mut numbers: Vec<u8> = (1..=9).collect();

        numbers.shuffle(&mut thread_rng());

        return numbers;
    }

    fn fill_recursively(self: &mut Self, empty_cells: &Vec<[usize; 2]>, nth_empty_cell: usize) -> bool
    {
        if nth_empty_cell >= empty_cells.len() {
            return true;
        }

        let [row_index, column_index]: [usize; 2] = empty_cells[nth_empty_cell];
        let numbers: Vec<u8> = self.generate_shuffled_vector();

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

    fn fill_grid(self: &mut Self) -> ()
    {
        let empty_cells: Vec<[usize; 2]> = self.find_empty_cells();

        self.fill_recursively(&empty_cells, 0);
    }

    fn count_solutions_recuresively(self: &mut Self, empty_cells: &Vec<[usize; 2]>, nth_empty_cell: usize, number_of_solutions: &mut u8) -> ()
    {
        if nth_empty_cell >= empty_cells.len() {
            *number_of_solutions += 1;
            return;
        }

        let [row_index, column_index]: [usize; 2] = empty_cells[nth_empty_cell];
        let numbers: Vec<u8> = self.generate_shuffled_vector();

        for number in numbers {
            if self.is_valid_placement(row_index, column_index, number) {
                self.grid[row_index][column_index] = number;
                self.count_solutions_recuresively(empty_cells, nth_empty_cell + 1, number_of_solutions);
                self.grid[row_index][column_index] = 0;
            }
        }
    }

    fn count_solutions(self: &mut Self, number_of_solutions: &mut u8) -> ()
    {
        let empty_cells: Vec<[usize; 2]> = self.find_empty_cells();

        self.count_solutions_recuresively(&empty_cells, 0, number_of_solutions);
    }

    fn has_unique_solution(self: &mut Self) -> bool
    {
        let number_of_solutions: &mut u8 = &mut 0;

        self.count_solutions(number_of_solutions);

        return *number_of_solutions == 1;
    }

    fn remove_some_cells(self: &mut Self, number_of_cells_to_remove: u8) -> ()
    {
        let indexes: Vec<usize> = (0..9).collect();
        let mut number_of_cells_to_remove: u8 = number_of_cells_to_remove;

        while number_of_cells_to_remove > 0 {
            let random_row_index: usize = *indexes.choose(&mut thread_rng()).unwrap();
            let random_column_index: usize = *indexes.choose(&mut thread_rng()).unwrap();

            if self.grid[random_row_index][random_column_index] != 0 {
                let backup_number: u8 = self.grid[random_row_index][random_column_index];
                self.grid[random_row_index][random_column_index] = 0;

                if self.has_unique_solution() {
                    number_of_cells_to_remove -= 1;
                } else {
                    self.grid[random_row_index][random_column_index] = backup_number;
                }
            }
        }
    }

    pub fn generate(self: &mut Self, difficulty: SudokuDifficulty) -> &mut Self
    {
        self.grid[0] = self.generate_shuffled_vector().try_into().unwrap();

        self.fill_grid();

        let possible_number_of_cells: Vec<usize> = match difficulty {
            SudokuDifficulty::ChildPlay => (40..45).collect(),
            SudokuDifficulty::Easy => (45..50).collect(),
            SudokuDifficulty::Medium => (50..55).collect(),
            SudokuDifficulty::Hard => (55..60).collect(),
            SudokuDifficulty::Expert => (60..65).collect(),
        };
        let number_of_cells_to_remove: u8 = *possible_number_of_cells.choose(&mut thread_rng()).unwrap() as u8;

        self.remove_some_cells(number_of_cells_to_remove);

        return self;
    }
}