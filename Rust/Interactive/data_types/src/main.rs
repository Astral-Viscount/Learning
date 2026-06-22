fn main() {
    //Scallar Types
    let i: i32 = 1_000; //1000 

    let a = 2.0; // f64

    let b: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let d: char = 'D'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    //Compund types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let j: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = j.0;

    let six_point_four = j.1;

    let one = j.2;

    let mut l: (i32, i32) = (1, 2);
    l.0 = 0;
    l.1 += 5;

    let e = [1, 2, 3, 4, 5];

    let f: [i32; 5] = [1, 2, 3, 4, 5];

    let g = [3; 5]; //[3, 3, 3, 3, 3]

    let h = [1, 2, 3, 4, 5];

    let first = h[0];
    let second = h[1];

    use std::io;

    let k = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = k[index];

    println!("The value of the element at index {index} is: {element}");
}
