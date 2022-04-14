#![allow(dead_code)]

use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};

#[allow(non_snake_case)]
pub fn Runner() {
    println!("Welcome, Noob");
    let sec_num = rand::thread_rng().gen_range(0..101);

    loop {
        println!("Enter a guess number!!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Faild to get input");

        let x: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // debug_line:: print!("The Guessed number is:: {}", guess)

        match x.cmp(&sec_num) {
            Ordering::Less => println!("{}", "Too Small".red()),
            Ordering::Equal => {
                println!("{}", "Correct".green());
                break;
            }
            Ordering::Greater => println!("{}", "Too big".red()),
        }
    }
}
