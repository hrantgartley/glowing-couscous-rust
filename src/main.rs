use rand::prelude::*;

fn main() -> () {
    welcome_message();
    let number_to_guess = generate_random_num();
    play_game(number_to_guess);
}

fn give_hint(number: i32, number_to_guess: i32) -> () {
    match number.cmp(&number_to_guess) {
        std::cmp::Ordering::Less => println!("Your guess is too low!"),
        std::cmp::Ordering::Greater => println!("Your guess is too high!"),
        std::cmp::Ordering::Equal => println!("You have guessed the number correctly"),
    }
}

fn play_game(number_to_guess: i32) -> () {
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

fn get_user_guess() -> i32 {
    loop {
        println!("Enter your guess:");
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

fn welcome_message() -> () {
    println!("Welcome to the guessing game...");
    println!("---------------------------------");
}
