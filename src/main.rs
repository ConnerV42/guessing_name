use rand::Rng;
use std::cmp::Ordering;
use std::io;

// Run `cargo doc --open` to view this crate's documentation.

fn main() {
    println!("Guess the number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // This is an example of shadowing, a feature of Rust that is
        // often used to convert a value from one type to another.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
               println!("Type a valid number!");
               continue;
            },
        };
            
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

