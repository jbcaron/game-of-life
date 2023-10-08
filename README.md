# Game of Life Simulator

This is a simple Rust program that simulates Conway's Game of Life. The Game of Life is a cellular automaton that evolves over discrete time steps based on a set of simple rules. It consists of a grid of cells, each of which can be either "alive" or "dead." The rules determine how the cells live, die, or reproduce in each generation.

## Usage

To use this program, you need to provide three command-line arguments:

1. `<width>`: The width of the board (number of columns).
2. `<height>`: The height of the board (number of rows).
3. `<percentage-alive>`: The initial percentage of cells that are alive (0-100).

Here's an example of how to run the program:

```bash
$ ./prgm 10 10 50
```

This command will create a 10x10 grid where approximately 50% of the cells are initially alive. The program will display the board's state and update it in real-time following the Game of Life rules.


## Rules

The Game of Life follows these rules for each generation:

- Any live cell with fewer than two live neighbors dies (underpopulation).
- Any live cell with two or three live neighbors lives on to the next generation.
- Any live cell with more than three live neighbors dies (overpopulation).
- Any dead cell with exactly three live neighbors becomes a live cell (reproduction).

## Implementation Details

The program is implemented in Rust and uses a custom Board struct to represent the game board. It initializes the board with random initial cell states based on the provided percentage. The step function computes the next generation's state, and the print function displays the current state in the terminal.

## Dependencies

This program uses the ftkit library for generating random numbers. You can find the library on crates.io with the version "^0.1.0". Make sure to add it to your Cargo.toml if it's not already there.