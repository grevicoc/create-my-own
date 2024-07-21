use std::io::{self, stdout, Write};
use rand::Rng;

fn main() {
    println!("Guess program executed.");
    let random_number = rand::thread_rng().gen_range(1..=11);

    loop {
        print!("Input your guess:");
        stdout().flush().expect("Error when flushing the stdout.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess : i32 = match guess.trim().parse() {
            Ok(value) => value,
            Err(_) => continue
        };

        match guess.cmp(&random_number) {
            std::cmp::Ordering::Less => println!("The number is greater than this."),
            std::cmp::Ordering::Greater => println!("The number is lower than this."),
            std::cmp::Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}
