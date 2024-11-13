use rand::Rng;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: russian_roulette <difficulty (1-6)> [--real]");
        process::exit(1);
    }

    let difficulty: u8 = args[1].parse().unwrap_or(0);

    if difficulty < 1 || difficulty > 6 {
        eprintln!("Difficulty must be between 1 and 6.");
        process::exit(1);
    }

    let is_real_mode = args.contains(&"--real".to_string());

    let winning_number = match difficulty {
        1 => rand::thread_rng().gen_range(2..=6),
        2 => rand::thread_rng().gen_range(2..=6),
        3 => rand::thread_rng().gen_range(3..=6),
        4 => rand::thread_rng().gen_range(4..=6),
        5 => rand::thread_rng().gen_range(5..=6),
        6 => 6, // Hardest mode: Only 6 wins
        _ => 0,  // Should never occur
    };

    // User input (simulate picking a number)
    println!("Choose a number between 1 and 6 based on the difficulty ({}):", difficulty);
    println!("(For fun, input a number here. It could be anything, for now we'll simulate it!)");

    let player_guess = rand::thread_rng().gen_range(1..=6); // Simulated guess for testing

    println!("You guessed: {}", player_guess);
    println!("The winning number was: {}", winning_number);

    if player_guess == winning_number {
        if is_real_mode {
            println!("Congrats! But in real mode... you survived the danger! 🥳");
        } else {
            println!("You win! 🎉 Great choice, you're safe and sound!");
        }
    } else {
        if is_real_mode {
            println!("Oh no! You lost! In real mode, this is your last chance... 😱");
            println!("But don't worry, it's just a game. You haven't lost anything important... yet!");
        } else {
            println!("You lost! 😬 Better luck next time. Run with --real for a more dangerous experience.");
        }
    }
}

