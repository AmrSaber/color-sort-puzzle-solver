# Color-Sort Puzzle Solver

Optimal solver for color-sort game (see [example](#example) and [game explanation](#game-explanation) sections below).

This project (written in Rust) efficiently solves the provided state of the game. The project both exports a library to solve the game and an executable that accepts a state of the game and outputs the steps to solve the game.

For a game of 10 (8 filled + 2 empty) containers -- the solver finds an optimal solution within 2 seconds.

You can enter the configuration of the game at any state (beginning or mid-game) and the solver will solve the game starting from that position.

## Game explanation

The game consists of a number of containers (usually vertical tubes), and each of these containers contains a number of mixed colors, the numbers are placed vertically and are separated from each other, and there is usually 2 empty tubes. With each move you pour the color on top of the tube into another empty tube or a tube with some space that has the same color on top.

To win the game you must sort all the colors in the tubes, i.e. each tube should be filled to its top with the same color.

See [example](#example) section below for a sample image for the game.

## Features

In addition to solving the game, there are also some additional features:

- Forcing a container to be filled: You can mark containers so that they must be filled in the final solution
- Forcing a container to be empty: You can mark containers so that they must be empty in the final solution

Of course, you can not mark the same container to be both full and empty. And providing an invalid constraints will simply result in a "No solution" output.

## Executable Input and Output

### Input

The executable reads any number of provided lines, each line representing a container. Each line must contain the colors included in that container from top to bottom. All colors must be separated by a single space. Anywhere in the line you can add a `*`, `+` or `-` but it must be separated from any other word, or else it will be considered part of the color's name.

Colors can be named anything as long as the naming is consistent, so they can be named :("red", "blue", "yellow"), ("rd", "bl", "ylw"), ("r", "b", "y"), or even ("a", "b", "c").

The name of each color can be anything except (`*`, `+` and `-`), and it cannot contain spaces.

To indicate that a certain container has some empty space, add `*` anywhere in its line as if it were a color. Add it a number of times equal to the number of empty spaces.

To indicate an empty container, you can just write one `*` in that line.

You can add `+` and `-` anywhere in the container description to indicate that it must be filled or emptied respectively. Adding them more than once has no effect, and if they are both in that line then only the last one will take effect.

### Output

- In case of invalid input, the app will exit with status code (1), and an error message representing what went wrong.
- In case the given state was already solved with project with output "State is already solved!" with exit state (0).
- In case there is no solution, the app will output "No solution!" with exit state (0).
- In case a solution was found, the app will:
  - Output first line: "Found solution in {number} steps:"
  - Then for each step: "- ({from_container}) -> ({to_container})", e.g. "- (07) -> (10)"
  - Then app will exit with status code (0)

## Example

The following puzzle ([source](http://kociemba.org/themen/waterball/colorsort.html)):

![puzzle image](./images/mixed.png)

Corresponds to the following input:

```
b r y g r
g b r g b
b y g y g
r b r y y
*
*
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
