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

    let a = (1,2); //元组
    println!("a is {:?}", a);

    let (a, b) = a; // 元组解构
    println!("a is {:?}, b is {:?}", a, b);

    let a: (i32, i64, char) = (32, 64, '1');
    println!("a is {:?}", a);

}
