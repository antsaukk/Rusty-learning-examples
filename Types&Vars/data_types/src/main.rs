#[allow(dead_code)]
#[allow(unused_variable)]

use std::mem;

fn main() {
    let a: u8 = 123;
    println!("a = {}", a);

    let mut b: i8 = 0;
    println!("b = {} before", b);
    b = 42;
    println!("b = {} after", b);

    let c = 123456789;
    println!("c = {}, c takes up {} bytes", c, mem::size_of_val(&c));

    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z*8);

    let d: char = 'x';
    println!("d = {}, d takes up {} bytes", d, mem::size_of_val(&d));

    let e = 2.5;
    println!("e = {}, takes up to {} bytes", e, mem::size_of_val(&e));

    let g: bool = false;
    println!("g = {}, takes up to {} bytes", g, mem::size_of_val(&g));
}
