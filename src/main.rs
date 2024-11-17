use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to 'Guess the Number'!");
    println!("I'm thinking of a number between 1 and 100.");

    // Generate a random number between 1 and 100
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // Create a mutable string to store user input
        let mut guess = String::new();

        // Read user input from standard input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the input to an unsigned 32-bit integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        // Compare the guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulations! You guessed the number!");
                break;
            }
        }
    }
}
