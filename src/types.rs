use crate::constants::*;

pub type SudokuCell = [usize; 2];
pub type SudokuRow = [u8; GRID_SIZE];
pub type SudokuGrid = [SudokuRow; GRID_SIZE];