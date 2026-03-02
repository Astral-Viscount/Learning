fn main() {
    //Output
    println!("Hello, world!");
    print!("Print");
    print!("Same line");
    
    //Variables
    let name = "John";
    let age = 30;
    println!("{} is {} years old.", name, age);
    
    //Mutable Variables
    let mut x = 5;
    println!("Before: {}", x);
    x = 10;
    println!("After: {}", x);
    
    //Defined Types
    let my_num: i32 = 5;          // integer
    let my_double: f64 = 5.99;    // float
    let my_letter: char = 'D';    // character
    let my_bool: bool = true;     // boolean
    let my_text: &str = "Hello";  // str
    
    //Constants
    const BIRTHYEAR: i32 = 1980; // Ok
    const BIRTHYEAR = 1980; // Error: missing type

    //If Statements
    let time = 20;
    let greeting = if time < 18 {
        "Good day."
        } else {
        "Good evening."
    };
    println!("{}", greeting);

    //Match
    let day = 4;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day."),
    }

    //Loop
    let mut count = 1;

    let result = loop {
        println!("Hello!");

        if count == 3 {
            break count; // Stop the loop and return the number 3
        }

        count += 1;
    };

    //While Loop
    let mut num = 1;

    while num <= 10 {
        if num == 6 {
            break;
        }
        println!("Number: {}", num);
        num += 1;
    }

    //For Loop
    for i in 1..=5 {
        println!("{}", i);
    }

    //Function
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    let sum = add(3, 4);
    println!("Sum is: {}", sum);

    //String
    let text1 = "Hello World".to_string();
    let text2 = String::from("Hello World");

    //Mutating String
    let mut greeting = String::from("Hello");
    greeting.push_str(" World");
    println!("{}", greeting); // Hello World

    //Adding 1 char
    let mut word = String::from("Hi");
    word.push('!');
    println!("{}", word); // Hi!

    //Combining Strings
    let s1 = String::from("Hello");
    let s2 = String::from("World!");
    let s3 = String::from("What a beautiful day!");
    let result = format!("{} {} {}", s1, s2, s3);
    println!("{}", result);

    let result = s1 + " " + &s2 + " " + &s3; //You can only add a &str to a String with +
    println!("{}", result);

    //String Length
    let name = String::from("John");
    println!("Length: {}", name.len()); // 4

    //Ownership
    let a = String::from("Hello");
    let b = a;

    // println!("{}", a); Error: a no longer owns the value
    println!("{}", b); // Ok: b now owns the value

    //Simple Types
    let a = 5;
    let b = a;
    println!("a = {}", a);  // Works
    println!("b = {}", b);  // Works

    //Cloning to keep the initial Ownership
    let a = String::from("Hello");
    let b = a.clone(); // Now both have the same value

    println!("a = {}", a);  // Works
    println!("b = {}", b);  // Works

    //Borrowing (No need to clone)
    let a = String::from("Hello");
    let b = &a;

    println!("a = {}", a);
    println!("b = {}", b);

    let mut name = String::from("John");
    let name_ref = &mut name;
    name_ref.push_str(" Doe");

    println!("{}", name_ref); // John Doe

    //Arrays
    let mut numbers = [1, 2, 3, 4, 5];
    numbers[0] = 10;
    println!("The new first number is: {}", numbers[0]);

    println!("This array has {} elements.", numbers.len());

    println!("{:?}", numbers); //Prints the entire array

    //Vectors
    let mut fruits = vec!["apple", "banana", "orange"];
    fruits[0] = "grape";
    println!("New first fruit: {}", fruits[0]);

    fruits.push("cherry");

    fruits.pop();

    fruits.insert(0, "apple");
    fruits.remove(0);

    println!("There are {} fruits.", fruits.len());

    let fruits = vec!["apple", "banana", "orange"];
    for fruit in &fruits { // Use &fruits to borrow the vector instead of moving it
        println!("I like {}.", fruit);
    }

    //Tuple
    let person = ("John", 30, true);
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Is active: {}", person.2);

    //Unpacking a tuple
    let person = ("Jenny", 45, false);
    let (name, age, active) = person;

    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Active: {}", active)

    //HashMap 
    // Import HashMap before main func
    use std::collections::HashMap;

    fn main() {
        // Create a HashMap called capitalCities
        let mut capitalCities = HashMap::new();

        // Add keys and values (Country, City)
        //HashMaps require keys to be unique. Inserting the same key again will overwrite the old value.
        capitalCities.insert("England", "London");
        capitalCities.insert("Germany", "Berlin");
        capitalCities.insert("Norway", "Oslo");

        println!("{:?}", capitalCities);
    }

    //Accessing values
    let mut capitalCities = HashMap::new();

    capitalCities.insert("England", "London");
    capitalCities.insert("Germany", "Berlin");
    capitalCities.insert("Norway", "Oslo");

    if let Some(city) = capitalCities.get("England") {
        println!("The capital of England is {}.", city);
    } else {
        println!("England is not in the map.");
    }

    //Updating values
    let mut capitalCities = HashMap::new();

    capitalCities.insert("England", "London");
    capitalCities.insert("England", "Berlin");

    println!("{:?}", capitalCities);

    //Removing values
    let mut capitalCities = HashMap::new();

    // Add keys and values (Country, City)
    capitalCities.insert("England", "London");
    capitalCities.insert("Germany", "Berlin");
    capitalCities.insert("Norway", "Oslo");

    // Remove the key "England"
    capitalCities.remove("England");

    println!("{:?}", capitalCities);

    //Looping
    let mut capitalCities = HashMap::new();

    // Add keys and values (Country, City)
    capitalCities.insert("England", "London");
    capitalCities.insert("Germany", "Berlin");
    capitalCities.insert("Norway", "Oslo");

    // Loop through the HashMap
    for (country, city) in &capitalCities {
        println!("The capital of {} is {}.", country, city);
    }

    //Structs
    //You can think of a struct like a mini-database for one thing, like a person with a name and age
    struct Person {
    name: String,
    age: u32,
    }

    let mut user = Person {
    name: String::from("John"),
    age: 35,
    };

    user.age = 36; // Change value of age
    println!("Name: {}", user.name);
    println!("Updated age: {}", user.age);

    //Enum 
    /* Enums are useful when you want to represent a value that can only be one of a set of options
       - like days of the week, directions, or results like success and error.*/
    enum LoginStatus {
    Success(String),
    Error(String),
    }

    fn main() {
        let result1 = LoginStatus::Success(String::from("Welcome, John!"));
        let result2 = LoginStatus::Error(String::from("Incorrect password"));

        match result1 {
            LoginStatus::Success(message) => println!("Success: {}", message),
            LoginStatus::Error(message) => println!("Error: {}", message),
        }
    }
}   
