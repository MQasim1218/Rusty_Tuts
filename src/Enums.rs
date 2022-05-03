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

    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    fn inbuilt_options() {
        let x1 = Some(45);
        let str1 = Some(String::from("GoodBye, Mars"));
        let no_num: Option<i32> = None;

        let res = x1.unwrap_or(0) + no_num.unwrap_or(0);
    }
}

pub fn Runner() {}
