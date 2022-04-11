#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

// use colored::*;

mod VariablesAndMutability {
    pub fn mut_check() {
        let x = 20;
        // x = 6;

        // Shadowing used here to reuse a previously declared variable name for another variable by
        // declaring another variable with the same name...
        let mut x = 34;
        x = 44;
        print!("Current Value of Y is:: {}", x);

        // Constants
        const WORTH: u64 = 5_000_000_000_000;
        /*
            constants have the following differences from immutable variables.
                1. Can't be set to mutable usig mut keyword.
                2. Dtype `Must` be declared at declaration.
                3. Cannot be assigned from a function (i.e. return value).
        */
    }
}

mod DataTypes {
    // use core::arch;

    pub fn dtypes_static() {
        // Number Data_Tpyes..
        let x1 = 33; // Default int declaration
        let x2: u32 = 33; // Declaring an unsigned 32bit integer.
        let x3: f32 = 54.8; // Floating type vals
                            // let x3: arch = 64; // Infer datatype at runtime based on the underlying - Check its working plss

        // Arthemetic operators
        let res = 45.0 - 33.0;
        let res = 65.0 / 2.0;
        let res = 23 - 65;
        let res = 36 % 7;

        //Boolean and chars
        let b1 = true;
        let ch1 = 'G';
    }
    pub fn dtypes_compound() {
        /*
            Tuples,
            Arrays,
            Vectors - Latrr in course!!,
            Maps, dicts, HashMaps and More!! (these are external downloadable crates if you dint want to self define them.)
        */

        // Tuples - fixed_length, multi-datatype `bags`
        let t1 = ("Qasim", 24);
        let (name, age) = t1; // Destructuring a Tuple!!
        print!("Your name is {}", name);
        println!(" and your age is {}.", t1.1);

        // Arrays - fixed_length, same-type objects!!
        let ar1 = [1, 2, 3];
        println!("Value at index 2:: {}.", ar1[2]); // Indexing an array.. It throws an index out of bounds exception!!

        let ar_zeros = [0; 8]; // Declare arrays with existing values!!
        print!("Value at index 4:: {}.", ar_zeros[4]);
    }
}

mod FuncsAndControlFlow {
    use rand::Rng;
    use std::io;

    pub fn tester() {
        let sec = rand::thread_rng().gen_range(1..21);
        println!("The Secret Number is:: {}", sec);

        for i in 0..3 {
            let mut guess = String::new();

            println!("Enter a number");
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to get input");

            let guess: i32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            match guess.cmp(&sec) {
                std::cmp::Ordering::Less => println!("Too Less"),
                std::cmp::Ordering::Equal => println!("Correct"),
                std::cmp::Ordering::Greater => println!("Too High"),
            }
        }
    }
    pub fn loops() {
        // simpeloop with a break
        let mut counter = 1;
        loop {
            if counter == 11 {
                println!("Quitting the Loop");
                break;
            } else {
                println!("Current Value:: {}", counter)
            }
            counter += 1;
        }

        println!("\nsimpeloop that returns a value");
        let mut counter = 1;
        let c = loop {
            if counter == 11 {
                break counter; // Add the value tobe returned infront of the break keyword
            }
            counter += 1;
        };
        println!("Value of C is: {}", c);

        // Two more loop types - For and While - Empty bodies below.
        for itr in 1..20 {}

        let ar_zeros = &mut [0; 10];
        // Quick demonstration of mapping inner_vals on a mutable array to computed calues using iter_mut()
        for elem in ar_zeros.iter_mut() {
            *elem += 1;
        }

        // Empty while loop body
        while false {
            print!("Impossible")
        }
    }
}

pub fn Runner() {
    // VariablesAndMutability::mut_check();
    // DataTypes::dtypes_static();
    // DataTypes::dtypes_compound();
    FuncsAndControlFlow::loops()
}
