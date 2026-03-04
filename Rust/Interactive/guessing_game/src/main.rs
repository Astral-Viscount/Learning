use std::io;

fn main() {
    println!("Guess:");

    let mut guess= String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Error");

    println!("Your guessed number: {guess}")
}
