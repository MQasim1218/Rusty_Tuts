#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(non_camel_case_types)]

mod vectors {
    use std::vec;

    pub fn code() {
        /*
            vects_vs_arys();
            vect_indexes();
            iterating_vects();
            single_value_types_workaround()
        */
    }

    fn single_value_types_workaround() {
        #[derive(Debug)]
        enum shape {
            Circle(u16),
            Rectangle(u16, u16),
            Square(u16),
        }

        let mut vec1: Vec<shape> = Vec::new();

        vec1.push(shape::Circle(12));
        vec1.push(shape::Square(12));
        vec1.push(shape::Rectangle(8, 8));

        // When reading values from a enum vector, matching is required
        let f = match &vec1[2] {
            shape::Circle(x) => shape::Circle(*x),
            shape::Square(x) => shape::Square(*x),
            shape::Rectangle(x, y) => shape::Rectangle(*x, *y),
        };

        println!("Shape is {:?}", f);
    }

    fn iterating_vects() {
        let mut vec1 = vec![3, 2, 5, 6, 8, 3, 9, 12];
        vec1.push(17); // Pushing some more elements
        vec1.remove(3); // Removing value at index 3

        // Mutable iter
        // Only use reference, dont borrow
        println!("Iterating vectors -- Immutable ");
        for i in &vec1 {
            println!("value: {}", i)
        }

        // Immutable Iter
        // Using a mutable reference this time!
        println!("\nIterating vectors -- Mutable (+= 25)");
        for i in &mut vec1 {
            // * is the derefernce operator, more on this latrr!!
            *i += 25;
            println!("value: {}", i)
        }
    }

    fn vect_indexes() {
        /*
            Two methods!!
            Indexing with index number or use get method.
        */
        let mut vec1 = vec![1, 2, 3, 4, 5, 6];

        /*
            Getting the value from the vector!!
            After getting the value, we can push to the vec as the value
            taken is copied to the variable.
        */
        let third = vec1[2];
        vec1.push(12);
        println!("{:?}", third);

        let sec = &vec1[1]; // Immutable borrow
        vec1.push(22); // Mutable borrow!!
                       // println!("This is the second value of the vector {}", sec);
                       // Error as mut and immut borrow takes place.

        // Indexing with get method
        if let Some(val) = vec1.get(8) {
            println!("The value at index 3 is {}", val);
        } else {
            println!("The index is incorrect!!");
        }
    }

    fn vects_vs_arys() {
        let mut ary1 = [1, 2, 3, 4, 5];
        let mut vec1: Vec<i32> = Vec::new();
        vec1.push(1);
        vec1.push(2);
        vec1.push(3);
        vec1.push(4);
        vec1.push(5);

        println!("Array: {:?}", ary1);
        println!("Vector: {:?}", vec1);
        println!();

        ary1 = [12, 12, 15, 17, 12]; // The internal values can be changed but the length remains unchanged!!
        ary1[4] = 23;
        vec1[4] = 23;

        println!("Array {:?}", ary1);
        println!("Vector {:?}", vec1);
        println!();

        vec1.push(54);

        println!("Vector can grow in length!!");
        println!("Array {:?}", ary1);
        println!("Vector {:?}", vec1);
        println!();
    }
}

mod hashmaps {
    pub fn code() {}
}

mod strings {
    pub fn code() {
        // In Rust, elements are stored as collections of UTF-8 encoded bytes: more on this later!!
    }
}

pub fn runner() {
    vectors::code()
}
