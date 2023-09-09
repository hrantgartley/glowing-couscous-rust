#![allow(dead_code)]

use rand::prelude::*;
use std::io::Write;
use std::time::Instant;

fn main() -> () {
    let now = Instant::now();
    let number_to_guess = generate_random_num();
    let valid = validate_bounds(number_to_guess);
    if valid {
        welcome_message();
        play_game(number_to_guess);
        exit_message();
    } else {
        println!("Not valid");
    }
    let time_e = now.elapsed();
    // convert times_e to minutes if it is greater than 60 seconds
    if time_e.as_secs() > 60 {
        let minutes = time_e.as_secs() / 60;
        println!("Time elapsed: {} minutes", minutes);
    } else {
        println!("Time elapsed: {} seconds", time_e.as_secs());
    }
}

fn give_hint(number: i32, number_to_guess: i32) {
    match number.cmp(&number_to_guess) {
        std::cmp::Ordering::Less => println!("Your guess is too low!"),
        std::cmp::Ordering::Greater => println!("Your guess is too high!"),
        std::cmp::Ordering::Equal => println!("You have guessed the number correctly"),
    }
}

fn play_game(number_to_guess: i32) {
    let mut count = 0;

    loop {
        let user_guess = get_user_guess();
        count += 1;

        give_hint(user_guess, number_to_guess);

        if user_guess == number_to_guess {
            println!("Correctly guessed {} in {} tries", number_to_guess, count);
            break;
        }
    }
}

fn validate_bounds(number: i32) -> bool {
    (10..=250).contains(&number)
}

fn get_user_guess() -> i32 {
    loop {
        print!("Enter your guess: ");
        std::io::stdout().flush().expect("Failed to flush stdout"); // Ensure the prompt is displayed immediately

        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        }
    }
}

fn generate_random_num() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(10..250)
}

fn welcome_message() {
    println!("Welcome to the guessing game...");
    println!("---------------------------------");
}

fn exit_message() {
    println!("Thanks for playing the guessing game.")
}

fn generate_odd() -> i32 {
    let mut rng = rand::thread_rng();
    let odd = rng.gen_range(1..100);
    if odd % 2 != 0 {
        odd
    } else {
        odd + 1
    }
}

fn generate_even() -> u32 {
    let mut rng = rand::thread_rng();
    let even = rng.gen_range(1..100);
    if even % 2 == 0 {
        even
    } else {
        even + 1
    }
}

fn hint_even(number: i32) {
    if number % 2 == 0 {
        println!("The number is even");
    } else {
        println!("The number is odd");
    }
}

fn limited_guesses() {
    let mut tries = 0;
    while tries < 5 {
        tries += 1;
        println!("You have {} tries left", 5 - tries);
    }
}
