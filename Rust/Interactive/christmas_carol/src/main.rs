fn main() {
    let lyrics: [&str; 12] = ["A partridge in a pear tree", "Two turtle doves and", "Three french hens", "Four calling birds", "Five golden rings", "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking", "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];

    for i in 0..12 {

        if i == 0 {
            println!("On the 1st day of Christmas, my true love sent to me");
            println!("{}", lyrics[i])
        }
        else if i == 1 {
            println!("On the 2nd day of Christmas, my true love sent to me");
            println!("{}", lyrics[i])
        }
        else if i == 2{
            println!("On the 3rd day of Christmas, my true love sent to me");
            println!("{}", lyrics[i])
        }
        else {
            println!("On the {}th day of Christmas, my true love sent to me", (i + 1));
            for j in (0..(i+1)).rev() {
                println!("{}", lyrics[j]);
            }
        }
    }
}
