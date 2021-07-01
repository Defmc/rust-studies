extern crate rand;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut attempts: u16 = 1;
    let num: u8 = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();

        print!("Enter the guess: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading input");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            _ => continue,
        };

        match guess.cmp(&num) {
            Ordering::Less => println!("Very Low"),
            Ordering::Equal => {
                println!(
                    "Congratulations! You win the game at {}º attempt!",
                    attempts
                );
                break;
            }
            Ordering::Greater => println!("Very high"),
        }
        attempts += 1;
    }
}
