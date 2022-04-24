// Implementing a Rectangle example
#![allow(unused_variables)]
#![allow(dead_code)]

mod Structs_Practice {
    pub fn Runner() {
        // Defining structs
        #[derive(Debug, Clone)]
        struct Cat {
            name: String,
            age: u32,
            date_of_birth: String,
            color: String,
        }

        // This is a default Cat object.
        let def_cat = Cat {
            name: String::from("Kitty"),
            age: 1,
            date_of_birth: String::from("01/01/2000"),
            color: String::from("White"),
        };

        let kitten = Cat {
            name: String::from("TakaMana"),
            age: 12,
            date_of_birth: String::from("12/04/2020"),
            color: String::from("Ginger"),
        };

        // This cat is exactly the same as default_cat.

        println!("kitten: \n{:#?}", kitten);
        println!("Kitten name: {}", kitten.name);
        println!("Kitten age: {}", kitten.age);
        println!("Kitten color: {}", kitten.color);
        println!("Kitten DOB: {}", kitten.date_of_birth);

        let cat1 = Cat {
            name: String::from("Chuzee"),
            ..def_cat.clone() // take the rest values from this object.
        };
        println!("{:#?}", cat1);

        // Tuple structs
        let x = (32, 32);

        // Impl blocks on structs
        let x1 = (32, 54.54, 54.54);
        struct Color(i32, f64, f64);
        struct Gradient(i32, f64, f64);

        // All three are different and can't be passed for each other.

        // Methods vs associated functions
        impl Cat {
            // This is an associated function.
            fn new_cat(def_cat: Cat) -> Cat {
                Cat { ..def_cat }
            }
        }

        let kitten = Cat::new_cat(def_cat.clone());
        println!("Kitten {:#?}", kitten)
    }
}

mod Rect_Example {

    #[derive(Debug, Clone)]
    struct Rectangle {
        height: u32,
        width: u32,
    }
    impl Rectangle {
        fn new(dims: (u32, u32)) -> Rectangle {
            Rectangle {
                height: dims.0,
                width: dims.1,
            }
        }

        fn new_def(rect: &Rectangle) -> Rectangle {
            Rectangle { ..rect.clone() } // Whatever is sent, clone and use!!
        }

        fn get_area(&self) -> u32 {
            self.width * self.height
        }
    }

    pub fn Runner() {
        let rect = Rectangle {
            height: 20,
            width: 30,
        };

        println!("Area of rect: {}", rect.get_area());

        let dims = (43, 54);
        let rect = Rectangle::new(dims);
        println!("Rectangle 2 {:#?}", rect);
        println!("Area of rectangle_2: {}", rect.get_area());

        let rect = Rectangle::new_def(&rect);
        println!("Rectangle 2 {:#?}", rect);
        println!(r#"Area of rectangle_3: {}"#, rect.get_area());
    }
}

pub fn Runner() {
    // Structs_Practice::Runner();
    Rect_Example::Runner();
}
