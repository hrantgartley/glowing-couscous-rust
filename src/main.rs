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

fn loop_until_done(mut sim: i32, num: i32) {
    while sim != num {
        let mut count = 1;
        count += 1;
        sim = generate_simulated_random_num();
        give_hint(sim, num);
        println!("Guessing again...");
        println!("The number guessed was {}", sim);
        if sim == num {
            println!("Correctly guessed {} in {} tries", num, count)
        }
    }
}

fn generate_random_num() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(10..250)
}

fn print_prompt_with_number() {
    let mut rng = rand::thread_rng();
    let mut start = generate_random_num();
    let mut end = generate_random_num();
    let number_to_guess = generate_random_num();
    let simulated_random_num = rng.gen_range(10..250);
    loop_until_done(simulated_random_num, number_to_guess);
    validate_bounds(&mut start, &mut end);
}

fn validate_bounds(start: &mut i32, end: &mut i32) {
    let mut rng = rand::thread_rng();
    let number_to_guess = generate_random_num();

    if number_to_guess > *end {
        println!("{} is bigger than {}... re-rolling", number_to_guess, *end);
        *end = rng.gen_range(10..250)
    } else if number_to_guess < *start {
        println!(
            "{} is bigger than {}... re-rolling",
            number_to_guess, *start
        );
        *start = rng.gen_range(10..250)
    }
}

fn generate_simulated_random_num() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(10..250)
}
