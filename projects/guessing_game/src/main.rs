use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hello, welcome to the guess the number game.!");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Guess the number. Input a number.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read.");

        let guess: u16 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low."),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("You win.");
                break;
            }
        }
    }
}
