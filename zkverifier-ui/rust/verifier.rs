use std::io::{self, Read};

fn main() {
    // Read proof and public inputs from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    // For now, just check if the input contains "valid"
    if input.contains("valid") {
        println!("{{\"result\":\"valid\"}} ");
    } else {
        println!("{{\"result\":\"invalid\"}} ");
    }
}
