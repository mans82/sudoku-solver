# Sudoku Solver
This is a small Sudoku solver program, written in Rust. It takes a sudoku problem saved in a `.txt` file, and prints all solutions to it.

It implements a graph-like DFS search to generate possible solutions. After filling in an empty cell in the table, it tries to fill in the next empty cell with an approriate value. If it fails to find an appropriate value for a cell, it backtracks and tries the next appropriate value for the previous empty cell. If it reaches a complete table (that is, with no empty cell), it returns the table as a solution.

The solver logic implements the `Iterator` trait, which allows for iterating through solutions, and using numerous useful default methods the `Iterator` trait provides (`skip`, `take`, `collect`, etc.). To allow implementing the solution logic as an `Iterator`, it keeps the DFS stack using a `Vec` to save the search state between calls to `next`. In recursive impelementations of DFS search, usually the call stack is used as such.

## Example

```bash
$ cargo run -- --version
Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/sudoku-solver --version`
sudoku-solver v0.1.0
$ cat >> input.txt << EOF
> XX1XXXXX5
XXXX34XXX
X5XXX19X6
X2X6XXXX3
X3XXXXX5X
765XX8X9X
9XX4XXX3X
XXX71XXX9
813XXXXX4
> EOF
$ cargo run -- input.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/sudoku-solver input2.txt`
 => Solution 1:
┌───┬───┬───┐ ┌───┬───┬───┐ ┌───┬───┬───┐
│ 2 │ 8 │ 1 │ │ 9 │ 6 │ 7 │ │ 3 │ 4 │ 5 │
├───┼───┼───┤ ├───┼───┼───┤ ├───┼───┼───┤
│ 6 │ 9 │ 7 │ │ 5 │ 3 │ 4 │ │ 2 │ 1 │ 8 │
├───┼───┼───┤ ├───┼───┼───┤ ├───┼───┼───┤
│ 3 │ 5 │ 4 │ │ 8 │ 2 │ 1 │ │ 9 │ 7 │ 6 │
└───┴───┴───┘ └───┴───┴───┘ └───┴───┴───┘
┌───┬───┬───┐ ┌───┬───┬───┐ ┌───┬───┬───┐
│ 1 │ 2 │ 9 │ │ 6 │ 7 │ 5 │ │ 4 │ 8 │ 3 │
├───┼───┼───┤ ├───┼───┼───┤ ├───┼───┼───┤
│ 4 │ 3 │ 8 │ │ 1 │ 9 │ 2 │ │ 6 │ 5 │ 7 │
├───┼───┼───┤ ├───┼───┼───┤ ├───┼───┼───┤
│ 7 │ 6 │ 5 │ │ 3 │ 4 │ 8 │ │ 1 │ 9 │ 2 │
└───┴───┴───┘ └───┴───┴───┘ └───┴───┴───┘
┌───┬───┬───┐ ┌───┬───┬───┐ ┌───┬───┬───┐
│ 9 │ 7 │ 2 │ │ 4 │ 8 │ 6 │ │ 5 │ 3 │ 1 │
├───┼───┼───┤ ├───┼───┼───┤ ├───┼───┼───┤
│ 5 │ 4 │ 6 │ │ 7 │ 1 │ 3 │ │ 8 │ 2 │ 9 │
├───┼───┼───┤ ├───┼───┼───┤ ├───┼───┼───┤
│ 8 │ 1 │ 3 │ │ 2 │ 5 │ 9 │ │ 7 │ 6 │ 4 │
└───┴───┴───┘ └───┴───┴───┘ └───┴───┴───┘
```

## Building
To build the project using `cargo`:
```bash
$ cargo build --release
```

You can find the built binary in `./target/release/sudoku-solver`.

Instead of manually building the project, you can directly run the project by running (assuming `input.txt` is the input file):
```bash
$ cargo run --release -- input.txt
```

