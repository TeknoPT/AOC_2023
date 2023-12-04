# Day 3: Gear Ratios

## Overview

Upon reaching a gondola lift station to ascend to the water source, you discover the gondola is malfunctioning. An engineer Elf needs help identifying a missing engine part, and later, the correct gear to replace.

## Part One: Identifying the Missing Part

**Title:** Engine Schematic Analysis

The engine schematic contains numbers and symbols, where any number adjacent to a symbol (excluding periods) is a "part number."

**Task:** Calculate the sum of all part numbers in the engine schematic.

**Example Schematic:**

```

467..114..
..._......
..35..633.
......#...
617_......
.....+.58.
..592.....
......755.
...$.\*....
.664.598..

```

In this example, the sum of the part numbers is 4361.

**Solution:**

To find the sum of all part numbers in the actual engine schematic:

```bash
cd Day3
cargo run --release part1
```

This command will execute the Rust program for Part One, outputting the total sum of part numbers.

## Part Two: Correcting the Gear Ratio

**Title:** Gear Ratio Calculation

A gear is any `*` symbol adjacent to exactly two part numbers. The gear ratio is the product of these two numbers.

**Task:** Find the gear ratio of every gear and add them all up.

**Example Schematic:**

```
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
```

In this example, the sum of the gear ratios is 467835.

**Solution:**

To calculate the sum of all gear ratios in the engine schematic:

```bash
cd Day3
cargo run --release part2
```

Running this command for Part Two will output the total sum of the gear ratios.

## Input File

The puzzles for Day 3 are based on a specific engine schematic. Access the input file for these challenges here: [Day 3 Input](input).

## Reflection

Completing Day 3's puzzles brought challenges in pattern recognition and algorithmic thinking, furthering my exploration in Rust programming.

---

Return to the [main README](../README.md) for an overview of all days.

This `README.md` provides a detailed overview of Day 3's challenges, including the context, tasks, examples, and solutions. It also includes instructions on how to run the code for each part and a link to the input file. Remember to update the link if your file structure or naming convention differs.
