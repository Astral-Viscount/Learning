use std::io;

fn main() {
    println!("Enter temperature: ");
    let temp: f32 = loop {
        let mut input = String::new();
    
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Not a Temperature! Please enter a temperature!");
                continue
            }
        };
    };
    
    println!("Temp: {temp}");
}
