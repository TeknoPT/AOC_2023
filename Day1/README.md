# Day 1: Trebuchet?!

## Overview

Today's challenge involves investigating an issue with global snow production. The Elves have provided a map marking fifty critical locations. To restore snow operations, all these locations must be checked by December 25th.

Each day of the Advent calendar presents two puzzles, with the second unlocking upon completion of the first. Solving each puzzle grants a star.

## Part One

**Title:** Calibration Trouble

The Elves have a calibration document for the trebuchet, which is crucial for your journey to the sky. However, a young Elf has altered the document with art, making it difficult to read the calibration values.

**Task:** Determine the sum of calibration values from the document. Each line's value is the combination of the first and last digits.

**Example:**

```
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
```

In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

**Solution:** You need to run the code

## Part Two

**Title:** Enhanced Calibration

It turns out that some digits are spelled out in letters (one, two, three, etc.). This requires a reevaluation of the calibration values.

**Task:** Redetermine the sum of calibration values, considering spelled-out numbers as valid digits.

**Example:**

```
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
```

In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

**Solution:** You need to run the code

## Input File

The puzzles for Day 1 are based on a specific set of data. You can find the input file for these challenges here: [Day 1 Input](input).

## Running the Code

Navigate to the Day 1 directory and run the solution:

```bash
cd Day1
cargo run --release
```

This will execute the Rust program for Day 1 with the provided input file.

## Reflection

Upon completion of these puzzles, I've earned two gold stars and gained new insights into problem-solving with Rust.

---

Return to the [main README](../README.md) for an overview of all days.
