fn main() {
    let a = 5;
    let mut b = a;
    b += 1;

    let a = Box::new([0; 1_000_000]); //to allocate this in a heap to make a pointer instead of copying like above
    let b = a;

    let a_num = 4;
    make_and_drop();
    
    let first = String::from("Ferris");
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");
}

fn make_and_drop() {
    let a_box = Box::new(5);
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}