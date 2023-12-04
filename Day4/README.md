# Day 4: Scratchcards

## Overview

Upon reaching Island Island via gondola, you encounter an Elf struggling with a pile of scratchcards. Your task is to analyze these scratchcards, first to calculate their points and then to determine how many more scratchcards they yield.

## Part One: Calculating Scratchcard Points

**Title:** Scratchcard Analysis

Each scratchcard lists winning numbers and the numbers you have. Points are calculated based on matches with the winning numbers.

**Task:** Determine the total points value of all scratchcards.

**Example:**

- `Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53` (8 points)
- `Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19` (2 points)
- `Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36` (0 points)

**Solution:**

To find the total points value:

```bash
cd Day4
cargo run --release part1
```

This command will run the Rust program for Part One, providing the total points value of all scratchcards.

## Part Two: Winning More Scratchcards

**Title:** Scratchcard Multiplication

Now, you must calculate how many scratchcards are won based on the number of matching numbers, including copies of scratchcards.

**Task:** Determine the total number of scratchcards you end up with, including originals and copies.

**Example:**

- `Card 1` wins four cards (cards 2, 3, 4, 5)
- `Card 2` wins two cards (cards 3, 4)
- Original and copied cards can win more cards

**Solution:**

To calculate the total number of scratchcards:

```bash
cd Day4
cargo run --release part2
```

Running this command for Part Two will display the total number of scratchcards, including all originals and copies.

## Input File

The puzzle input contains the list of scratchcards. Access it here: [Day 4 Input](input).

## Reflection

Solving Day 4's puzzles required logical thinking and a thorough understanding of pattern matching and recursion, enhancing my skills in Rust programming.

---

Return to the [main README](../README.md) for an overview of all days.

This `README.md` provides a detailed overview of Day 4's challenges, including the tasks, examples, and solutions for each part. It also includes instructions on how to run the code for each part and a link to the input file. Remember to update the link if your file structure or naming convention differs.

```

```
