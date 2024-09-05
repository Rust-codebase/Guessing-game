use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the Number");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read user input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please add a valid input");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("you won");
                break;
            }
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
        }
        println!("You guessed: {guess}");
    }
}
