use rand::Rng;
use std::io;
fn main() {
    loop {
        let mut how_many = String::new();
        println!("How many numbers doo you want to guess");
        io::stdin()
            .read_line(&mut how_many)
            .expect("Error while reading line");

        let num_guess: u8 = how_many.trim().parse().expect("Erro reading input ");
        let mut guess = Vec::new();
        for _i in 0..=num_guess {
            guess.push(rand::rng().random_range(1..=40));
        }
        println!("The correct answer is {guess:?}");
    }
}
