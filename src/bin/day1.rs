use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day1.txt").unwrap();

	let mut floor = 0;
	let mut part2 = None;
	for (i, c) in file.chars().enumerate() {
		if c == '(' {
			floor += 1;
		} else if c == ')' {
			floor -= 1;
		}

		if floor == -1 && part2.is_none() {
			part2 = Some(i + 1);
		}
	}

	println!("Day 1 part 1: {}", floor);
	println!("Day 1 part 2: {}", part2.unwrap());
}