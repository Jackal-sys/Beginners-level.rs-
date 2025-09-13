use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let guess = rand::rng().random_range(1..=40);
    loop {
        println!("The correct answer is: {guess}");
        println!("Enter a desired number");
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Error while reading line");
        let num2: i32 = num.trim().parse().expect("Error while reading line");
        let message = match num2.cmp(&guess) {
            Ordering::Greater => "Too High",
            Ordering::Less => "Too Low",
            Ordering::Equal => {
                println!("You have got it correct");
                break;
            }
        };
        println!("Here are the results: {message}");
    }
}
