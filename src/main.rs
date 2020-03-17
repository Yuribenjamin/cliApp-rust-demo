use std::io;

fn main() {
    println!("Enter your bloody name: ");

    let mut name = String::new();
    io::stdin().read_line(&mut name);
    println!("Welcome bloody {}", name);
}
