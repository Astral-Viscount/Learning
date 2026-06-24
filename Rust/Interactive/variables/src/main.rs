fn main() {
    let mut x: i32 = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("A constant of 3 hours in second: {}", THREE_HOURS_IN_SECONDS);

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let spaces = "   ";
    let spaces = spaces.len();

    //error code: not allowed to mutate a variable’s type
    // let mut spaces = "   ";
    // spaces = spaces.len();

}

