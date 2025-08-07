use std::io;

fn main() {
    let mut name = String::new();
    println!("what is your name?");
    match io::stdin().read_line(&mut name){
        Ok(_) => println!("Hello, {}", name.trim()),
        Err(message) => println!("There was an error: {message}")
    }
}