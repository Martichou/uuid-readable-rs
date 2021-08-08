use uuid::Uuid;
use uuid_readable_rs::{generate, generate_from, generate_inverse, short};

fn main() {
    let uuid = Uuid::parse_str("29b0c30d-0a73-4036-8d5e-91ac43bca845").unwrap();
    let x = generate_from(uuid);
    println!("Converted uuid = {}", x);
    println!("Original uuid = {}", &uuid);
    println!("Decoded uuid = {}\n", generate_inverse(&x).unwrap());

    println!("Random full-lenght = {}", generate());
    println!("Random short-lenght = {}", short());
}
