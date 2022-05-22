mod generics_n_traits {
    pub fn code() {
        generic_funcs();
        struct_generics();
    }

    fn struct_generics() {
        #[derive(Debug)]
        struct Point<T, U> {
            x: T,
            y: U,
        }

        impl<T, U> Point<T, U> {
            fn mixup<A, B>(self, oth: Point<A, B>) -> Point<T, B> {
                Point {
                    x: self.x,
                    y: oth.y,
                }
            }
        }

        let p1 = Point { x: 10, y: 20 };
        let p2 = Point { x: 10.0, y: 20.0 };

        let p3 = p1.mixup(p2);
        println!("{:?}", p3);
    }

    fn generic_funcs() {
        println!("{}", getLargest(vec![61, 22, 83, 44, 65, 068, 97, 87, 94]));
        println!(
            "{}",
            getLargest(vec!['a', 't', 'h', 'L', 'Z', 'a', 'q', 'Y', 'w'])
        );
    }

    fn getLargest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
        let mut largest: T = list[0];
        for val in list {
            if val > largest {
                largest = val;
            }
        }
        largest
    }
}

pub fn runner() {
    generics_n_traits::code()
}
