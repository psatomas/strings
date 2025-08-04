fn main() {
    let pirate = "Bloodhook";

    let sailer = String::from(pirate);

    let bad_guy = pirate.to_string();

    println!("{pirate} and {sailer} and {bad_guy}");
    
    let first_initial = &pirate[0..1];
    println!("{first_initial}");
}
