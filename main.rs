#include <iostream>
#include <fstream>
#include <string>

using namespace std;

int main() {
    string username;
    string password;

    // Input from user
    cout << "Enter username: ";
    cin >> username;

    cout << "Enter password: ";
    cin >> password;

    // Open file to append
    ofstream file("users.txt", ios::app);
    if (file.is_open()) {
        file << username << "," << password << endl;
        file.close();
        cout << "User registered!" << endl;
    } else {
        cerr << "Error opening file." << endl;
    }

    return 0;
}
use std::fs::OpenOptions;
use std::io::{self, Write};
use argon2::{Argon2, PasswordHasher};
use password_hash::SaltString;
use rand::rngs::OsRng;

fn main() {
    let mut username = String::new();
    let mut password = String::new();

    println!("Enter username:");
    io::stdin().read_line(&mut username).expect("Failed to read username");
    let username = username.trim();

    println!("Enter password:");
    io::stdin().read_line(&mut password).expect("Failed to read password");
    let password = password.trim();

    if username.len() < 3 || password.len() < 6 {
        println!("Username must be at least 3 characters and password at least 6 characters.");
        return;
    }
    // Generate a random salt
    let salt = SaltString::generate(&mut OsRng);

    // Create Argon2 hasher instance with default parameters
    let argon2 = Argon2::default();

    // Hash password with the salt
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Password hashing failed")
        .to_string();

    // Open file to append
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("users_secure.txt")
        .expect("Cannot open file");

    // Write username and hashed password to file
    writeln!(file, "{},{}", username, password_hash).expect("Failed to write to file");

    println!("User registered securely.");
}
