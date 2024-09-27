# SUDOKU in Rust

To use rust in your local environment, start [here](https://www.rust-lang.org/tools/install).

## Usage Instruction

Run the following command in the terminal to build the execution file.

```shell
cargo build
```

To test project functionality, run the following command,

```shell
cargo test
```

To initiate the program use the following command.

```shell
cargo run
```

After running this command, you will be prompted to select puzzle difficulty.

```text
Select sudoku difficulty: (Default: Medium)
 1) Child Play
 2) Easy
 3) Medium
 4) Hard
 5) Expert
```

By Choosing the difficulty, the Sudoku puzzle will be generated.

```text
0 0 7 0 5 0 9 4 0 
4 8 0 0 0 9 0 7 0
0 6 0 0 0 1 0 0 8
0 0 1 0 0 7 0 0 0
5 0 9 0 0 6 0 0 0
6 0 8 3 2 5 0 1 0
0 5 0 0 0 0 7 0 0
8 2 0 0 0 0 0 0 0
0 0 0 2 0 3 0 0 0
```
Note: The number 0 (zero) represents an empty cell.
