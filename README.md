# Wordle CLI

This is a command-line implementation of Wordle, inspired by the popular New York Times word game.

## Description

Wordle CLI is a Rust-based game that challenges players to guess a five-letter word in six attempts. After each guess, the game provides feedback using color-coded letters:

- Green: The letter is in the correct position
- Yellow: The letter is in the word but in the wrong position
- Gray: The letter is not in the word

## Setup

1. Ensure you have Rust installed on your system.
2. Clone this repository.
3. Navigate to the project directory.

## Word List

The game reads words from a file located at `src/words.txt`. You can customize this word list by replacing the content of this file with your own list of five-letter words.

**Note:** The current `words.txt` file contains Italian words and may not be completely accurate. Feel free to replace it with a more comprehensive or language-specific word list.

## How to Play

1. Run the game using `cargo run`.
2. The game will randomly select a word from `src/words.txt`.
3. You have 6 attempts to guess the word.
4. After each guess, the game will display the result using color-coded feedback.
5. Keep guessing until you find the word or run out of attempts.

## Dependencies

- `std`: Standard library for file and input operations
- `rand`: For generating random numbers

## Customization

You can easily customize the game by modifying the `src/words.txt` file. Simply replace the contents with your own list of five-letter words, one word per line.

Enjoy playing Wordle in your terminal!
