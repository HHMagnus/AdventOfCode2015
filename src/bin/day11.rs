use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day11.txt").unwrap();

	let mut chars = file.chars().collect::<Vec<_>>();
	let idx = chars.len()-1;

	while !valid(&chars) {
		shift(&mut chars, idx);
	}

	let part1 = chars.iter().collect::<String>();
	println!("Day 11 part 1: {}", part1);
	
	shift(&mut chars, idx);

	while !valid(&chars) {
		let idx = chars.len()-1;
		shift(&mut chars, idx);
	}

	let part2 = chars.iter().collect::<String>();
	println!("Day 11 part 2: {}", part2);
}

fn shift(cs: &mut Vec<char>, idx: usize) {
	let c = cs[idx];
	if c == 'z' {
		cs[idx] = 'a';
		shift(cs, idx-1);
	} else {
		cs[idx] = char::from_u32(c as u32 + 1).unwrap();
	}
}

fn valid(input: &Vec<char>) -> bool {
	if !input.as_slice().windows(3).any(|x| (x[0] as u32 + 1) == (x[1] as u32) && (x[1] as u32 + 1) == (x[2] as u32)) {
		return false;
	}

	if input.contains(&'i') || input.contains(&'o') || input.contains(&'l') {
		return false;
	}

	non_overlapping(input) >= 2
}

fn non_overlapping(chars: &Vec<char>) -> usize {
	let mut count = 0;

	let mut i = 0;
	while i < chars.len()-1 {
		let x1 = chars[i];
		let x2 = chars[i + 1];
		if x1 == x2 {
			count += 1;
			i += 1;
		}
		i += 1;
	}
	
	count
}