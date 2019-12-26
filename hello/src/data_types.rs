use std::mem;

pub fn run() {
    // unsigned
    let a:u8 = 123; // immutable
    println!("a = {}", a);

    let mut b:i8 = 0; // mutable
    println!("b = {}", b);

    b = 42;
    println!("b = {}", b);

    let mut c = 123456789; // let rust infer data type
    println!("c = {}, bytes: {}", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {} after modification", c);

    let z:isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {} takes up {} bytes, {}-bit OS",
                z,
                size_of_z,
                size_of_z * 8
            )

}