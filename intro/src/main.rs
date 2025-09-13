use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Welcome to my guessing game");
    loop {
        println!("Enter a valid number");
        let correct = rand::rng().random_range(1..=20);
        println!("{correct}");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_e) => continue,
        };
        println!("You guessed : {guess}");
        match guess.cmp(&correct) {
            Ordering::Greater => println!("Too High"),
            Ordering::Less => println!("Too Low"),
            Ordering::Equal => {
                println!("You have guessd correct");
                break;
            }
        }
    }
}
