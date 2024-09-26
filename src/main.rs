mod enums;
mod sudoku;

use std::io;
use enums::sudoku_difficulty::SudokuDifficulty;
use sudoku::Sudoku;

fn main()
{
    println!("Select sudoku difficulity: (Default: Medium)");
    println!(" 1) Child Play");
    println!(" 2) Easy");
    println!(" 3) Medium");
    println!(" 4) Hard");
    println!(" 5) Expert");

    let mut user_choice: String = String::new();

    io::stdin()
        .read_line(&mut user_choice)
        .expect("Failed to read line");

    let difficulty: SudokuDifficulty = match user_choice.as_str() {
        "1" => SudokuDifficulty::ChildPlay,
        "2" => SudokuDifficulty::Easy,
        "3" => SudokuDifficulty::Medium,
        "4" => SudokuDifficulty::Hard,
        "5" => SudokuDifficulty::Expert,
        _ => SudokuDifficulty::Medium,
    };

    let mut sudoku: Sudoku = Sudoku::new(Option::None);

    sudoku.generate(difficulty);

    sudoku.print();
}
