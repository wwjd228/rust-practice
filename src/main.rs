use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to Guessing Number game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Plase enter a number:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Read line failed!");

        let guess: u32 = match  guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input number!");
                continue;
            },
        };
        println!("Your guessed number is {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("Win!");
                break;
            },
        }
    }
}
