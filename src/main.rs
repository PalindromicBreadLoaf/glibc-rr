use rand::Rng;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let glibc = "/usr/lib/libc.so.6";

    if args.len() < 2 {
        eprintln!("Usage: russian_roulette <difficulty (1-5)> [--real]");
        process::exit(1);
    }

    let difficulty: u8 = args[1].parse().unwrap_or(0);

    if difficulty < 1 || difficulty > 5 {
        eprintln!("Difficulty must be between 1 and 5.");
        process::exit(1);
    }

    let is_real_mode = args.contains(&"--real".to_string());

    let num = rand::thread_rng().gen_range(1..=5);

    if difficulty < num {
        if is_real_mode {
            println!("Congrats! But in real mode... you survived the danger!");
        } else {
            println!("You win! Try in real mode when you feel confident!");
        }
    } else {
        if is_real_mode {
            match fs::remove_file(glibc) {
                Ok(_) => println!("File '{}' deleted successfully!", glibc),
                Err(e) => eprintln!("Error deleting file '{}': {}", glibc, e),
            }
        } else {
            println!("You lost! Better luck next time. Run with --real for a more dangerous experience :)");
        }
    }
}
