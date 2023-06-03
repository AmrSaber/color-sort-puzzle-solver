# Color-Sort Puzzle Solver

Optimal solver for color-sort game (see [example](#example) and [game explanation](#game-explanation) sections below).

This project (written in Rust) efficiently solves the provided state of the game. The project both exports a library to solve the game and an executable that accepts a state of the game and outputs the steps to solve the game.

For a game of 10 (8 filled + 2 empty) containers -- the solver finds an optimal solution within 2 seconds.

You can enter the configuration of the game at any state (beginning or mid-game) and the solver will solve the game starting from that position.

This solver will either find the optimal solution, or print that there is no solution. There is also an option to find suboptimal solution, which is a lot quicker to calculate.

## Game explanation

The game consists of a number of containers (usually vertical tubes), and each of these containers contains a number of mixed colors, the numbers are placed vertically and are separated from each other, and there is usually 2 empty tubes. With each move you pour the color on top of the tube into another empty tube or a tube with some space that has the same color on top.

To win the game you must sort all the colors in the tubes, i.e. each tube should be filled to its top with the same color.

See [example](#example) section below for a sample image for the game.

## Features

In addition to solving the game, there are also some additional features:

- Forcing a container to be filled: You can mark containers so that they must be filled in the final solution
- Forcing a container to be empty: You can mark containers so that they must be empty in the final solution

Of course, you can not mark the same container to be both full and empty. And providing an invalid constraints will simply result in a "No solution" output.

## Executable

### Installing

To install the solver, you will first have to [install Rust](https://www.rust-lang.org/tools/install) (it's pretty easy to use, and will make you look cool), then in the terminal type:

```bash
cargo install color-sort-solver --git https://github.com/AmrSaber/color-sort-puzzle-solver
```

**OR** if you want to have the binary itself (for some reason), then you will need to clone the repo and build it manually (you can also cross-compile it for other OS/architectures using something like [cross](https://github.com/cross-rs/cross)):

```bash
cd some-folder
git clone git@github.com:AmrSaber/color-sort-puzzle-solver.git
cd color-sort-puzzle-solver

# Of course you will still need to install rust
cargo build -r
```

Then you will find the binary under `target/release` called `color-sort-solver`.

Once you have the installed the package, you can then run it and give it some input and it will solve it (see sections below for more details)

```
color-sort-solver
b r y g r
g b r g b
b y g y g
r b r y y
*
*
```

> Note: if you enter the input in the terminal, you will need to send end-of-output signal once you are done (ctrl+d for Unix, ctrl+z for windows)

And it will output the solution

```
Found solution in 17 steps:
- (04) -> (05)
- (01) -> (04)
- (01) -> (05)
- (03) -> (06)
- (01) -> (03)
- (02) -> (01)
- (02) -> (06)
- (02) -> (05)
- (02) -> (01)
- (04) -> (02)
- (04) -> (05)
- (03) -> (04)
- (03) -> (01)
- (03) -> (04)
- (01) -> (03)
- (02) -> (06)
- (01) -> (05)
```

You can also save your input in a file, and redirect the content of that file as input for the command, e.g. if input is in file `input.txt` then you can do:

```bash
color-sort-solver < input.txt
```

### Uninstalling

If you installed the package using `cargo install` method, then you can simply uninstall it using

```
cargo uninstall color-sort-solver
```

### Input

The executable reads an arbitrary number of lines, each line representing a container (any empty line is ignored). Each line must contain the colors included in that container from top to bottom. All colors must be separated by 1 or more spaces.

Colors can be named anything as long as the naming is consistent, so they can be named :("red", "blue", "yellow"), ("rd", "bl", "ylw"), ("r", "b", "y"), or even ("super-color-1", "super-color-2", "lovely-color-1000").

The name of each color can be anything except (`*`, `+` and `-`), and it cannot contain spaces.

To indicate that a certain container has some empty space, add `*` anywhere in its line as if it were a color. Add it a number of times equal to the number of empty spaces.

To indicate an empty container, you can just write one `*` in that line.

You can add `+` and `-` anywhere in the container description to indicate that it must be filled or emptied respectively. Adding them more than once has no effect, and if they are both in that line then only the last one will take effect.

There are 3 rules for input:

- All containers must be of same capacity.
- Each color must be present a number of times equal to container capacity.
- Number of colors must be less than or equal to the number of the containers.

Executable also accepts an optional flag `-f` (stands for `fast` option), which will output suboptimal (but fast) solution. It is a lot faster than the optimal solutions (e.g. in some cases, the optimal solution takes 1.5 seconds, and the suboptimal takes 0.05 seconds).

### Output

- In case of invalid input, the app will exit with status code (1), and an error message representing what went wrong.
- In case the given state was already solved with project with output "State is already solved!" with exit state (0).
- In case there is no solution, the app will output "No solution!" with exit state (0).
- In case a solution was found, the app will:
  - Output first line: "Found solution in {number} steps:"
  - Then for each step: "- ({from_container}) -> ({to_container})", e.g. "- (07) -> (10)"
  - Then app will exit with status code (0)

### Example

The following puzzle ([source](http://kociemba.org/themen/waterball/colorsort.html)):

![puzzle image](./images/mixed.png)

Corresponds to the following input:

```

b r y g r
g b r g b
b y g y g
r b r y y

-
-

```

And the solver generates the following output:

```

Found solution in 17 steps:

- (04) -> (05)
- (01) -> (04)
- (01) -> (05)
- (03) -> (06)
- (01) -> (03)
- (02) -> (01)
- (02) -> (06)
- (02) -> (05)
- (02) -> (01)
- (04) -> (02)
- (04) -> (05)
- (03) -> (04)
- (03) -> (01)
- (03) -> (04)
- (01) -> (03)
- (02) -> (06)
- (01) -> (05)

```

```

```
