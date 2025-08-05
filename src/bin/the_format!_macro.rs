fn main(){
    let first_name = String::from("Sylvester");
    let last_name = String::from("Stallone");

    let icon = format!("{0} {1} {0}", first_name, last_name);
    println!("{icon}");
    println!("{first_name}");
    println!("{last_name}");
}