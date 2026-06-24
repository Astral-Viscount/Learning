use std::io;

fn main() {
    println!("Enter number to find nth Fibonacci number: ");
    let num: usize = loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        match input.trim().parse() {
            Ok(n) => break n,
            Err(_) => {
                println!("Please input an number!");
                continue;
            }
        };
    };

    println!("{num}th Fibonacci number: {}", fib(num));
}

fn fib(n: usize) -> usize {
    if n <= 1 {
        return n;
    }

    return fib(n - 1) + fib(n - 2);
}
