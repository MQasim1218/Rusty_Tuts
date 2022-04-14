#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod Basics {
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

    fn takes_ownership(s: &str) {}
    pub fn ownership_in_funcs() {
        println!("Onwer Ship in functions now!!\n");
        let s1 = "Qasim";
        takes_ownership(s1);
        print!("String Slice => {}", s1); // Slices are copied, thus ownership is not moved!!
    }
    pub fn borrowing_reference_passing() {}
    pub fn mut_borrow() {}
}

pub fn Runner() {
    // Basics::scopes();
    // Basics::var_n_data();
    Basics::ownership_in_funcs();
}
