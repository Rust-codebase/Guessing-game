use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the Number");

    // Generate a random number between 1 and 100
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read user input");

        // Parse the input guess as a u32
        let guess: u32 = guess.trim().parse().expect("failed to parse");
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
