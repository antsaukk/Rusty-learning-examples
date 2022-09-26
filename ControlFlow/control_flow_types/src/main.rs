#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_must_use)]

//use rand::Rng;
use std::io::stdin;

enum State {
	Locked,
	Failed,
	Unlocked
}

fn combination_lock() {
	let code = String::from("1234");
	let mut state = State::Locked;
	let mut entry = String::new();

	loop {
		match state {
			State::Locked => {

				let mut input = String::new();

				match stdin().read_line(&mut input) {
					Ok(_) => {
						entry.push_str(&input.trim_end());

					}
					Err(_) => continue
				}

					if entry == code {
						state = State::Unlocked;
						continue;
					}

					if !code.starts_with(&entry) {
						state = State::Failed;
					}
			}

			State::Failed => {
				println!("FAILED");
				entry.clear();
				state = State::Locked;
				continue;
			}

			State::Unlocked => {
				println!("UNLOCKED");
				return;
			}

		}
	}
}

fn match_statement(country_code: i32) -> &'static str {

	let country = match country_code {
		44 => "UK",
		46 => "Sweden",
		7  => "Russia",
		1..=1000 => "unknown",
		_ => "invalid"
	};

	country
}

fn main() {
    println!("{}", match_statement(44));
    combination_lock();
}
