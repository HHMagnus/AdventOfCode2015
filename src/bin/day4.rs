use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day4.txt").unwrap();

	let mut part1 = 0;

	loop {
		part1 += 1;
		let hash = format!("{:?}", md5::compute(format!("{}{}", file, part1)));
		let mut chars = hash.chars();
		if chars.next().unwrap() != '0' || chars.next().unwrap() != '0' || chars.next().unwrap() != '0' || chars.next().unwrap() != '0' || chars.next().unwrap() != '0' {
			continue;
		}
		break;
	}

	println!("Day 4 part 1: {}", part1);

	let mut part2 = 0;

	loop {
		part2 += 1;
		let hash = format!("{:?}", md5::compute(format!("{}{}", file, part2)));
		let mut chars = hash.chars();
		if chars.next().unwrap() != '0' || chars.next().unwrap() != '0' || chars.next().unwrap() != '0' || chars.next().unwrap() != '0' || chars.next().unwrap() != '0' || chars.next().unwrap() != '0' {
			continue;
		}
		break;
	}

	println!("Day 4 part 2: {}", part2);
}