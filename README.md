# Guessing Game Readme

This is a simple guessing game implemented in Rust. The game generates a random number between 10 and 250 and challenges the player to guess it. This README provides an overview of the code and its functionality.

## Table of Contents

- [Getting Started](#getting-started)
- [How to Play](#how-to-play)
- [Code Structure](#code-structure)

## Getting Started

Before you can play the game, you need to have Rust installed on your system. If Rust is not installed, you can download and install it from [Rust's official website](https://www.rust-lang.org/).

Once Rust is installed, you can follow these steps to run the game:

1. Clone this repository or download the code.

2. Open a terminal or command prompt and navigate to the directory where the code is located.

3. Build and run the code using the following command:

   ```bash
   cargo run
   ```

# How to Play

The game is straightforward. You'll be asked to guess a randomly generated number between 10 and 250 (could be changed). Here is how the game works.

1. When you start the game, you will see a welcome message.
2. You will then be prompted to enter your guess. Simply type the number and press Enter.
3. The game will provide feedback based on your guess.
   - If your guess to too low, you will be told that your guess is too low.
   - If your guess to too high, you will be told that your guess is too high.
   - If your guess is correct, you will be informed that your guess was correct.
4. Repeat steps 2 and 3 until you guess correctly.
5. When you correctly guess the number, the game will display the number you guessed and how many tries it took.
6. The game will exit.

# Code Structure

_Here's an overview of the code structure:_

    - `main()`: The main function that calls all other functions.
    - `welcome_message()`: Displays the welcome message when the game starts.
    - `generate_random_num()`: Generates a random number between 10 and 250 for the user to guess.
    - `play_game(number_to_guess)`: Implements the game loop, allowing the user to make guesses until they guess correctly.
    - `get_user_guess()`: Reads the player's input as a guess, handling invalid inputs.
    - `give_hint(number, number_to_guess)`: Provides feedback to the player about their guess (too low, high, or correct).

# Imports

- This program only uses the `rand()` to generate random numbers
