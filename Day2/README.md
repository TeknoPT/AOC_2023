Sure, here's a `README.md` file for Day 2 of your Advent of Code 2023 challenge in Rust:

````markdown
# Day 2: Cube Conundrum

## Overview

After being launched into the atmosphere, you land on Snow Island, where an Elf welcomes you. To pass the time, the Elf introduces you to a game involving red, green, and blue cubes. Your puzzle input contains information recorded from several games you played.

## Part One

**Title:** Cube Count Conundrum

The Elf wants to know which games could have been possible with a specific number of cubes: 12 red, 13 green, and 14 blue.

**Task:** Determine which games are possible with the given cube count and sum up their IDs.

**Example:**

```
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
```

In game 1, three sets of cubes are revealed from the bag (and then put back again). The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green cubes.

**Solution:** You need to run the code (sum of the IDs of possible games)

## Part Two

**Title:** Minimum Cube Set Challenge

The Elf asks for the fewest number of cubes of each color needed to make each game possible.

**Task:** Find the minimum set of cubes required for each game and calculate the sum of their powers (product of the number of red, green, and blue cubes).

**Example:**

```
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
```

- In game 1, the game could have been played with as few as 4 red, 2 green, and 6 blue cubes. If any color had even one fewer cube, the game would have been impossible.
- Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue cubes.
- Game 3 must have been played with at least 20 red, 13 green, and 6 blue cubes.
- Game 4 required at least 14 red, 3 green, and 15 blue cubes.
- Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.

The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together. The power of the minimum set of cubes in game 1 is 48. In games 2-5 it was 12, 1560, 630, and 36, respectively. Adding up these five powers produces the sum 2286.

**Solution:** You need to run the code (sum of the powers of the minimum sets)

## Input File

Find the input data for these puzzles here: [Day 2 Input](input).

## Running the Code

Navigate to the Day 2 directory and run the solution:

```bash
cd Day2
cargo run --release
```
````

This will execute the Rust program for Day 2 using the provided input file.

## Reflection

Completing these puzzles earned two gold stars and provided a unique challenge in problem-solving and data analysis using Rust.

---

Return to the [main README](../README.md) for an overview of all days.

```

This README gives a detailed overview of Day 2's challenges, including the task, examples, and solutions for each part. It also includes instructions on how to run the code and a link to the input file. Remember to update the link if your file structure or naming convention differs.
```
