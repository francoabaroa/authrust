use authrust::*;
use std::io::{stdin, Read};

fn main() {
    let mut connection = establish_connection();

    let mut username = String::new();
    let mut email = String::new();
    let mut password = String::new();

    println!("What would you like your username to be?");
    stdin().read_line(&mut username).unwrap();
    let username = username.trim_end(); // Remove the trailing newline

    println!("Enter your email:");
    stdin().read_line(&mut email).unwrap();
    let email = email.trim_end(); // Remove the trailing newline

    println!("Enter your password:");
    stdin().read_line(&mut password).unwrap();
    let password = password.trim_end(); // Remove the trailing newline

    let user = create_user(&mut connection, &username, &email, &password);
    println!("\nUser {} with id {} and email {} created.", username, user.id, email);
}
