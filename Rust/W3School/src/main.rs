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
    
}
