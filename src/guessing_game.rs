use rand::Rng;
use std::{cmp::Ordering, io};

pub fn Runner() {
    println!("Welcome, Noob");
    println!("Enter a guess number!!");

    let mut guess = String::new();

    let sec_num = rand::thread_rng().gen_range(0..101);
    loop {
        io::stdin()
            .read_line(&mut guess)
            .expect("Faild to get input");

        // debug_line:: print!("The Guessed number is:: {}", guess)
        let x: i32 = guess.trim().parse().expect("Expected a number!!");

        match x.cmp(&sec_num) {
            Ordering::Less => println!("Too Small"),
            Ordering::Equal => {
                println!("Correct");
                break;
            }
            Ordering::Greater => println!("Too big"),
        }
    }
}
