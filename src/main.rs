use std::io;

mod structs;  // Ensure `mod structs;` is declared
use structs::account::Account;
use structs::client::Client;

fn main() {

    let client = Client { 
        username: String::from("testuser"), 
        password: String::from("123"), 
        firstName:  String::from("John"), 
        lastName: String::from("Doe"), 
        email: String::from("John@example.com"), 
        accounts: Vec::new(),
    };

    println!("Welcome to your bank!");

    println!("Please Enter Username");

    let mut username = String::new();

    io::stdin()
    .read_line(&mut username)
    .expect("Username does not match anything in our records. Please try again.");

    println!("Please Enter Password");

    let mut password = String::new();

    io::stdin()
    .read_line(&mut password)
    .expect("Incorrect password. Please try again.");

    if username.trim() == client.username && password.trim() == client.password {
        println!("Welcome!");
    } else {
        println!("Wrong Credentials Try Again");
    }
}
