use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess:");
    let number = rand::thread_rng().gen_range(1..=100);
    // println!("Number: {number}");
    
    let mut score: u32 = 0; 

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
            Ordering::Less => {
                println!("Too small!");
                score += 1;
            }
            Ordering::Greater => {
                println!("Too big!");
                score += 1;
            }
            Ordering::Equal => {
                score += 1;
                println!("You guessed it! You win!");
                println!("Your Score {score}!");
                break;
            }
        }
    }
}
