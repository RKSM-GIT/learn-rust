use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to the guessing game");

    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please guess a number: ");
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: i32 = match input.trim().parse() {
            Ok(x) => x,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        match input.cmp(&secret) {
            Ordering::Less => println!("Guess higher!"),
            Ordering::Equal => {
                println!("You guessed correctly");
                break;
            },
            Ordering::Greater => println!("Guess lower")
        }
    }

    println!("You have won the game!");
}
