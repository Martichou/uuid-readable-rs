use uuid::Uuid;
use uuid_readable_rs::{generate, generate_from, generate_inverse, short};

fn main() {
    let uuid = Uuid::parse_str("08c60edc-1297-476c-a876-cef77a014757").unwrap();
    let x = generate_from(uuid);
    println!("Converted uuid = {}", x);
    println!();
    println!("Decoded uuid = {}", generate_inverse(&x).unwrap());
    println!("Original uuid = {}\n", &uuid);

    println!("Random full-lenght = {}", generate());
    println!("Random short-lenght = {}", short());
}
