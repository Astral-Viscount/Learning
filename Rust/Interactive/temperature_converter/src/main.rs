use std::io;

fn main() {
    println!("Enter temperature: ");
    let temp: f32 = loop {
        let mut input = String::new();
    
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
    
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Not a Temperature! Please enter a temperature!");
                continue
            }
        };
    };

    // println!("Temp: {temp}");
    
    println!("Choose the unit you want to convert to: Fahrenheit (F) or Celsius (C)");
    let choice = loop {

        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

        let input = input.trim().to_lowercase();

        if input == "f" || input == "c" || input == "fahrenheit" || input == "celsius" {
            break input;
        }
        else {
            println!("Not a Unit! Please enter the unit you want to convert to!");
        }
    };

    // println!("Choice: {choice}");

    if choice == "F" {
        println!("Temperature: {temp} Celsius.");
        println!("Converted to {} degrees Fahrenheit.", ((temp * 9.0/5.0) + 32.0));
    }
    else {
        println!("Temperature: {temp} Fahrenheit.");
        println!("Converted to {} degrees Celsius.", ((temp - 32.0) + 5.0/9.0));
    }

}
