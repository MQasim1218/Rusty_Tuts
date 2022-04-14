#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod Basics {
    use std::io;

    pub fn scopes() {
        let x1 = 32;
        {
            let x2 = 45.5;
        } // X2 is dropped here as its scope ends!!

        println!("The value of x1 is {}", x1);
        // println!("The value of x2 is {}", x2); Error: X2 not in this Scope
    }

    pub fn var_n_data() {
        let x1 = 5;
        let x2 = x1;

        println!("Values of X1 && X2 are:=> {}, {}", x1, x2);

        let str1 = "String Slice";
        let str2 = str1;
        println!("Str1:: {}, Str2:: {}", str1, str2);

        // String slices (&str) and primitive datatypes are cloned, thus no ownership error.

        let s1 = String::from("String 1");
        let s2 = s1; // Rust defaults to move (change ownership)
                     // println!("String 1 => {}, String 2 => {}", s1, s2);
                     // Err: S1 value is moved to S2, thus s1 is invalidated

        // Borrowing values instead of moving
        // Create a ptr that points to the existing value
        let s3 = &s2;
        println!("S2(Owner) => {}, S3(reference_borrowed) => {}", s2, s3);
        println!("S2(Owner) => {}, S3(reference_borrowed) => {}", s2, s3);

        // Mutable borrow
        let mut s4 = String::from("Hello, ");
        let s5 = &mut s4;
        {
            s5.push_str(" World");
            // println!("S4(Owner) => {}, S5(Mutable_borrow) => {}", &s4, s5); This doesnt work due to multiple borrowing
            // Sending val to a printstatement is also borrowing, which can lead to complicated things...
            println!("S5(Mutable_borrow) => {}", s5);
        }
        println!("Printing s4:: {}", &s4);
    }

    fn takes_ownership<T>(s: T) {}
    fn gives_own_ship<T>(val: T) -> T {
        println!("We have ownership of the value");
        val
    }

    pub fn ownership_in_funcs() {
        println!("Onwer Ship in functions now!!\n");
        let s1 = "Qasim";
        takes_ownership(s1);
        println!("String Slice => {}", s1); // Slices are copied, thus ownership is not moved!!

        let s2 = String::from(s1);
        takes_ownership(s2);
        // println!("Value of S2 is moved!! => {}", s2); Err: Value moved in the above line!!

        let s2 = String::from(s1); // Shadowing used here!! Reinstantiate the variable (to regain ownership!!)
        let s3 = gives_own_ship(s2);
        // print!("S2 => {}, S3 => {}", s2, s3); S2 has been borrowed, and returned to S3, thus it no longer exists!!
        println!("S3 => {}", s3);
    }

    fn problematic_example(val: String) -> (String, usize) {
        let len = val.len();
        (val, len)
    }
    fn less_tesious_method(val: &String) -> usize {
        val.len()
    }

    pub fn borrowing_reference_passing() {
        /*
            Rules for references!!
                1. No two mutable references in a scope!!
                2. Any number of immutable references!!
                3. No immutable_refs in scope in presence of mut_ref.
        */

        let str1 = String::from("Islamabad!!");
        // Here, we transfer the ownership to the function!!
        let (str2, mut len) = problematic_example(str1);
        println!("The lenght of String \'{}\' is :: {}", str2, len);

        println!("\nSame Work!! Less tedious way!!");
        len = less_tesious_method(&str2);
        println!("The lenght of String '{}' is {}", &str2, len);
    }

    fn greet_me(s: String) -> String {
        /*
            Longer method
            Steps:
                1. Create a new String.
                2. Get the input
                3. Create a third string by adding the prev 2 Strings.
                4. Return the last String.
        */
        let mut name = String::new(); // Step: 1
        io::stdin()
            .read_line(&mut name)
            .expect("Cant read the name"); // Step: 2

        let greeting = s + &name; // Step: 3
        greeting // Step: 4
    }

    pub fn simple_mutable_greeting(s: &mut String) {
        println!("Enter Your name: ");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Cant read the name");
        s.push_str(&name);
    }
    pub fn mut_borrow() {
        println!("Working with Mutable Borrows!!");
        let s1 = String::from("Hello, ");
        let greeting = greet_me(s1);
        print!("{}", greeting);

        let mut s2 = String::from("Hello, ");
        print!("Simpler Greeting\n");
        simple_mutable_greeting(&mut s2);
        println!("The Mutated String is {}", s2);

        // Cant make 2 mutable references of a Same Variable!!
        let s3 = &mut s2;
        let s4 = &mut s2;
        let s5 = &s2;

        // The error is Identified when we try to manipulate the references
        // (The lifetime is as long as the variable is in use) -
        // Dies before the scope is exited,
        // thus allows for another mut or immut reference!!).
        // println!("{} {} {}", s3, s4, s5) -- The error is encountered here!!
    }

    // fn getStr() -> &String {
    //     let s = String::new();
    //     // Since the String literal 'S' is a local variable, it is dropped from scope,
    //     // however the pointer is returned, thus a dangling pointer.
    //     &s
    // }

    // This is a demonstration on how to get a dangling pointer in seconds.
    pub fn dangling_ptr() {

        // let new_str = getStr();
        // A return ptr with no actual value associated in the backend.
    }

    fn first_word(str_val: &String) -> usize {
        let bytes = str_val.as_bytes();
        // This function converts the string characters to unicode numbers
        // and stores them as an array of integers.
        println!("{}", b' ');
        // Use iter().enumerate() to get index and value for each collection element.
        for (ind, &elem) in bytes.iter().enumerate() {
            // b' ' converts the value of ' ' to its unicode value(i-guess)!!
            if elem == b' ' {
                return ind;
            }
        }
        str_val.len() // If entire string is just one word
    }
    fn second_word_via_slicing(s: &String) -> &str {
        let bytes = s.as_bytes(); // Get the array of Character-Bytes
        let mut indxx = 0;
        for (ind, &elem) in bytes.iter().enumerate() {
            if elem == b' ' {
                // Check for an empty space.. A complete word till here
                indxx = ind;
                break;
            }
        }
        let s = &s[indxx + 1..];
        let bytes = s.as_bytes();

        for (ind, &elem) in bytes.iter().enumerate() {
            if elem == b' ' {
                return &s[..ind];
            }
        }

        &s[..] // Return entire string!!
    }

    pub fn Slicing() {
        /*
            Slices:
                1. Let user reference a contiguous part of a collection data.
                2. Donot take ownership of the actual data. Just refernece data.
                3. Tied to the Orignal data.!!
        */
        let mut s = String::from("Hello, ");
        simple_mutable_greeting(&mut s);

        let gr_slice = &s[0..]; // Get the entire slice, starting index 1.
        println!("{}", gr_slice);

        // Problem: Getting first word from a String!!
        let ind = first_word(&s);
        println!("The First word is {}", &s[..ind]); // Here, I'm using slicing, which is a bit cheating.

        let mut s = String::from("Hello, ");
        simple_mutable_greeting(&mut s);

        let sec_word = second_word_via_slicing(&s);
        println!("Second word via Slicing!! {}", sec_word);

        // Array Slice
        let a = [0; 10];
        let sl = &a[..5];
        println!("{:?}", sl)
    }
}

pub fn Runner() {
    // Basics::scopes();
    // Basics::var_n_data();
    // Basics::ownership_in_funcs();
    // Basics::borrowing_reference_passing();
    // Basics::mut_borrow();
    Basics::Slicing();
}
