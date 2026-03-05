use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess:");
    let number = rand::thread_rng().gen_range(1..=100);
    // println!("Number: {number}");

    loop {

        let mut guess= String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Error");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number: ");
                continue;
            }
        };

        println!("Your guessed number: {guess}");

        match guess.cmp(&number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You guessed it! You win!");
                break;
            }
        }
    }
}
