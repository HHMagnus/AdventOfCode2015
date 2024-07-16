use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day10.txt").unwrap();

	let mut result = file;
	for _ in 0..40 {
		result = apply(result);
	}

	println!("Day 10 part 1: {}", result.len());

	for _ in 0..10 {
		result = apply(result);
	}

	println!("Day 10 part 2: {}", result.len());
}

fn apply(file: String) -> String {
	let mut new = String::new();
	let mut chars = file.chars();
	let mut last = chars.next().unwrap();
	let mut count = 1;
	for x in chars {
		if x == last {
			count += 1;
		} else {
			new.push_str(&count.to_string());
			new.push(last);
			count = 1;
			last = x;
		}
	}
	new.push_str(&count.to_string());
	new.push(last);

	new
}