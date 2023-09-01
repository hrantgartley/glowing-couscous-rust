#![allow(dead_code)]
use rand::prelude::*;

fn main() {
    print_prompt_with_number();
}
fn give_hint(number: i32, number_to_guess: i32) {
    match number.cmp(&number_to_guess) {
        std::cmp::Ordering::Less => println!("Your guess is too low!"),
        std::cmp::Ordering::Greater => println!("Your guess is too high!"),
        std::cmp::Ordering::Equal => println!("You have guessed the number correctly"),
    }
}

fn valid(number_one: i32, number_two: i32) -> bool {
    number_one < 1000 && number_one > 0 && number_two < 1000 && number_two > 0
}

/*
FIXME: split this method into multiple methods
*/
fn generate_random_num() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(10..250)
}

fn print_prompt_with_number() {
    let mut rng = rand::thread_rng();
    let mut start: i32 = generate_random_num();
    let mut end: i32 = generate_random_num();
    let number_to_guess: i32 = generate_random_num();
    if number_to_guess > end {
        println!("{} is bigger than {}... re-rolling", number_to_guess, end);
        end = rng.gen_range(10..250)
    } else if number_to_guess < start {
        println!("{} is bigger than {}... re-rolling", number_to_guess, start);
        start = rng.gen_range(10..250)
    }
    let mut simulated_random_num: i32 = rng.gen_range(10..250);
    let mut count = 0;
    while simulated_random_num != number_to_guess {
        count += 1;
        simulated_random_num = rng.gen_range(10..250);
        give_hint(simulated_random_num, number_to_guess);
        println!("Guessing again...");
        println!("The number guessed was {}", simulated_random_num);
        if simulated_random_num == number_to_guess {
            println!("Correctly guessed {} in {} tries", number_to_guess, count)
        }
    }
    if !valid(start, end) {
        println!("These numbers are not within the constraints of the rubric... re-rolling");
        print_prompt_with_number();
    }
}
