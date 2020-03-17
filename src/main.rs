#![allow(non_snake_case)]
fn main() {
    let first = "Ibrahim".to_string();
    let last = "Ragab".to_string();

    say_name(first, last)
}

fn say_name(first: String, last: String) {
    println!("{} {}", first, last)
}
