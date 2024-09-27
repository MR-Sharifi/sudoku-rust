use crate::constants::*;

pub type SudokuCellIndex = [usize; 2];
pub type SudokuRow = [u8; GRID_SIZE];
pub type SudokuGrid = [SudokuRow; GRID_SIZE];