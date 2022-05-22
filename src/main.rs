#![allow(non_snake_case)]

mod Enums;
mod Err_handling;
mod RustOwnership;
mod collections;
mod guessing_game;
mod rust_concepts;
mod structs;
mod generics;

fn main() {
    println!("Hello, world!");
    // guessing_game::Runner();
    // Rust_Concepts::Runner()
    // RustOwnership::Runner();
    // structs::Runner();
    // Enums::Runner();
    // collections::runner();
    Err_handling::runner()
}
