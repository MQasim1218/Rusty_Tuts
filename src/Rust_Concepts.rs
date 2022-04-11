#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

// use colored::*;

mod variables_and_mutability {
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

mod data_types {
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

    pub fn dtypes_compound() {}
}

pub fn Runner() {
    variables_and_mutability::mut_check()
}
