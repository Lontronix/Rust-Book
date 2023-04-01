use std::io;
use std::cmp::Ordering;
use rand::Rng;

const ERROR_MESSAGE: &str = "please enter a number between 1 and 100";

fn main() {

    // start..=end (inclusive on lower and upper bound)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is {secret_number}");

    println!("Guess the number!");

    loop {

        println!("Please input your guess:");

       let mut guess = String::new();

       io::stdin()
           .read_line(&mut guess)
           .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                if num > 100 || num == 0 {
                    println!("{ERROR_MESSAGE}");
                    continue;
                } else {
                    num
                }
            },
            Err(_) => {
                println!("{ERROR_MESSAGE}");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
           Ordering::Less => println!("{guess} is too small!"),
           Ordering::Greater => println!("{guess} is too large!"),
           Ordering::Equal => {
               println!("You guessed the number!");
               break;
           }
        }
    }
}
