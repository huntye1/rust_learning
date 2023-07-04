use std::io;

fn main() {
    let mut n = String::new();
    println!("plz input number:");
    io::stdin().read_line(&mut n).expect("Failed to read input");
    let prec = n.trim().parse().expect("Failed to parse number");
    println!("pi is {:.prec$}", std::f64::consts::PI);
}
