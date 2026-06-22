use rand::Rng; //rand num gen
use std::cmp::Ordering; //for comparing (Less, Greater, Equal)
use std::io; //input/output library

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100); //calls the generator and pass range = (start..=end (inclusive)) 

    // println!("The secret number is: {secret_number}");
    let mut score: i32 = 0;

    loop {
        //infinite loop
        println!("Please input your guess.");

        let mut guess: String = String::new(); //makes an empty mutable string

        io::stdin()
            .read_line(&mut guess) //reference to guess and appends the user input to guess (immutable by default)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, //catch-all value
        };

        println!("You guessed: {guess}");

        //takes reference to whatever you wanna compare then does something based on which variant of Ordering was returned from the call to cmp
        match guess.cmp(&secret_number) {
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
                println!("You win! Your score: {score}");
                break;
            }
        }
    }
}
