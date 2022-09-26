#[allow(dead_code)]
#[allow(unused_variable)]

use std::mem;

struct Point {
	x: f64,
	y: f64
}

fn origin() -> Point {
	Point{x: 0.0, y: 0.0}
}

pub fn stack_n_heap() {
	let p1 = origin();
	let p2 = Box::new(origin());

	println!(
		"size of p1 = {} bytes\nsize of p2 = {} bytes",
		mem::size_of_val(&p1),
		mem::size_of_val(&p2));

	let p3 = *p2;

	println!("p3 = {}", p3.x);
}


fn main() {
    stack_n_heap();
}
