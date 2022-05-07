#![allow(dead_code)]
#![allow(unused_variables)]

mod vectors {

    pub fn code() {
        vects_vs_arys()
    }

    fn vects_vs_arys() {
        let ary1 = [1, 2, 3, 4, 5];
        let mut vec1: Vec<i32> = Vec::new();
        vec1.push(1);
        vec1.push(2);
        vec1.push(3);
        vec1.push(4);
        vec1.push(5);

        println!("{:?}", ary1);
        println!("{:?}", vec1);
    }
}

mod hashmaps {
    pub fn code() {}
}

mod strings {
    pub fn code() {}
}

pub fn runner() {
    vectors::code()
}
