#![allow(non_snake_case)]
fn main() {
    let first = "Ibrahim".to_string();
    say_name(&first);
    say_name(&first);
}

fn say_name(first: &String) {
    println!("{}", first)
}

/*function has been called with this value, we can no longer use this variable
 pass it by reference instead of by value using the & symbol and by updating the function's signature
*/
