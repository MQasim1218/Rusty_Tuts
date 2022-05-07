#![allow(dead_code)]
#![allow(unused_variables)]

mod enums {
    enum IpAddrKind {
        V4,
        V6,
    }

    enum Addr {
        V4(u8, u8, u8, u8),
        V6(u8, u8, u8, u8, u8, u8),
    }

    struct IpAddress {
        kind: IpAddrKind,
        addr: Addr,
    }

    fn enums_fn() {
        let ip1 = IpAddress {
            kind: IpAddrKind::V4,
            addr: Addr::V4(10, 101, 27, 192),
        };
        let ip2 = IpAddress {
            kind: IpAddrKind::V6,
            addr: Addr::V6(10, 20, 30, 40, 50, 60),
        };
    }
}

mod option_enum {

    pub fn matchin_states() {
        #[derive(Debug)]
        enum Cont {
            Europe,
            Asia,
            America(State),
            Africa,
            Australia,
            Antarctica,
            Arctic,
        }

        #[derive(Debug)]
        enum State {
            LosAngeles,
            California,
            NewYork,
            Texas,
            Florida,
        }

        fn match_st(cont: Cont) {
            match cont {
                Cont::Europe => println!("Hello from {:?}", cont),
                Cont::Asia => println!("Hello from {:?}", cont),
                Cont::America(state) => match state {
                    State::LosAngeles => println!("Hello from the American state {:?}", state),
                    State::California => println!("Hello from the American state {:?}", state),
                    State::NewYork => println!("Hello from the American state {:?}", state),
                    State::Texas => println!("Hello from the American state {:?}", state),
                    State::Florida => println!("Hello from the American state {:?}", state),
                },
                Cont::Africa => println!("Hello from {:?}", cont),
                Cont::Australia => println!("Hello from {:?}", cont),
                Cont::Antarctica => println!("Hello from {:?}", cont),
                Cont::Arctic => println!("Hello from {:?}", cont),
            }
        }

        match_st(Cont::Africa);
        match_st(Cont::America(State::NewYork))
    }

    pub fn inbuilt_options() {
        let x1 = Some(45);
        let str1 = Some(String::from("GoodBye, Mars"));
        let no_num: Option<i32> = None;

        let res = x1.unwrap_or(0) + no_num.unwrap_or(0);
        println!("The value of res is {}", res);
    }
}

mod ret_opt {

    fn ret_opt<T>(val: Option<T>) -> Option<T> {
        match val {
            Some(value) => Some(value), // For any value, return the value.
            _ => None,                  // For any other possible case
        }
    }

    pub fn runner() {
        println!(
            "Getting value of a fn that returns an option, {}",
            ret_opt(Some(50)).unwrap_or(0)
        );
    }
}

mod if_let_pattMatching {
    // If_let: shorter way to write match exprs when only matchig for one value
    pub fn runner() {
        // consider this!!
        let x = Some(3);
        match x {
            Some(5) => println!("Value of x is {}", x.unwrap()),
            Some(_) => println!("Not valid value"),
            None => println!("Not a value"),
        } // here we are checking is only x is 5, otherwise discard.. We can shorten the syntax using if_let_pattMatching!!

        if let Some(5) = x {
            println!("Correct value");
        } else {
            println!("Incorrect value");
        }
    }
}

pub fn Runner() {
    // option_enum::inbuilt_options();
    // ret_opt::runner();
    if_let_pattMatching::runner();
}
