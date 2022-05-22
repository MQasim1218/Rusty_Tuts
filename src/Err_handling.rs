#![allow(dead_code)]

mod res_enum {
    use core::panic;
    use std::{
        fs::{self, File},
        io::{self, ErrorKind, Read},
    };

    pub fn code() {
        // open_file();
        open_file_with_closures();
        match err_prop() {
            Ok(poem) => println!("{}", poem),
            Err(err) => panic!("We encountered the following error: {}", err.kind()),
        };
    }

    fn one_line_file_opener() -> Result<String, io::Error> {
        fs::read_to_string("poem_file.txt")
    }

    fn err_prop() -> Result<String, io::Error> {
        // Open a file and store its contents to a string
        // Verbose way of opening file, reading contents to string and returning

        let mut poem_file = File::open("poem_file.txt")?;
        let mut s = String::new();

        match poem_file.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(err) => Err(err),
        }
    }

    fn err_prop_simplified() -> Result<String, io::Error> {
        let mut str = String::new();
        File::open("poem_file.txt")?.read_to_string(&mut str)?;
        Ok(str)
    }

    fn open_file() {
        let F = File::open("hello.txt");

        if let Ok(F) = F {
            println!("File opened")
        } else {
            println!("Failed to open File. Creating a New file");
            match File::create("hello.txt") {
                Ok(F) => F,
                Err(err) => match err.kind() {
                    ErrorKind::AlreadyExists => panic!("The file alreadyExists"),
                    _ => panic!("Some other error occured while creating file"),
                },
            };
        }
    }

    fn open_file_with_closures() {
        File::open("poem_file.txt").unwrap_or_else(|err| {
            if err.kind() == ErrorKind::NotFound {
                let F = File::create("poem_file.txt")
                    .expect("The System crashed while trying to create a file at line 31");
                F
            } else {
                panic!("The File opening has encountered some error {}", err.kind())
            }
        });
    }
}

pub fn runner() {
    // panic!("Lats panic");
    res_enum::code()
}
