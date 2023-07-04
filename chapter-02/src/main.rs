use std::io;

fn main() {
    println!("Hello word");
    println!("Please input number");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    println!("your raw input is: {:?}.", input);
    let number: i64 = input.trim().parse().expect("input is not a number");
    println!("your input is: {}.", number);
}
