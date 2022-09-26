
fn option() {
	let x = 3.0;
	let y = 4.0;

	let result = 
		if y != 0.0 {Some(x/y)} else {None};

	match result {
		Some(z) => println!("{}/{} = {}", x, y, z),
		None => println!("Can not divide by 0")
	}

	if let Some(z) = result {
		println!("result = {}", z);
	}
}

fn main() {
    option();
}
