use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        println!();
        println!("Welcome to 'Guess the Number'!");
        println!("Choose a mode:");
        println!("1: Manual Guessing (You guess)");
        println!("2: Random Guessing (Computer guesses randomly)");
        println!("3: Binary-Search Guessing (Computer guesses systematically)");
        println!("4: To Quit");

        // Read the user's choice
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please restart the program and enter 1, 2, or 3.");
                return;
            }
        };

        // Generate a random secret number between 1 and 100
        let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

        match choice {
            1 => manual_guessing(secret_number),
            2 => random_guess(secret_number),
            3 => binary_search_guess(secret_number),
            4 => break,
            _ => println!("Invalid choice. Please restart the program and enter 1, 2, or 3."),
        }
    }
}

fn manual_guessing(secret_number: u32) {
    println!();
    println!("You chose Manual Guessing!");
    println!("I'm thinking of a number between 1 and 100.");
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        println!();
        println!("You guessed: {}", guess);

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

fn random_guess(secret_number: u32) {
    println!("You chose Random Guessing!");
    let mut rng = rand::thread_rng();
    loop {
        let guess = rng.gen_range(1..=100);
        println!("Random guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Random guessing succeeded: {} was the correct number!", guess);
                break;
            }
        }
    }
}

fn binary_search_guess(secret_number: u32) {
    println!("You chose Binary Search Guessing!");

    let mut low = 1;
    let mut high = 100;

    while low <= high {
        let mid = (low + high) / 2;
        println!("Binary search guess: {}", mid);

        match mid.cmp(&secret_number) {
            std::cmp::Ordering::Less => {
                println!("Too small!");
                low = mid + 1; // Narrow the range to the upper half
            }
            std::cmp::Ordering::Greater => {
                println!("Too big!");
                high = mid - 1; // Narrow the range to the lower half
            }
            std::cmp::Ordering::Equal => {
                println!("Binary search succeeded: {} was the correct number!", mid);
                break; // Exit the loop when the number is found
            }
        }
    }
}