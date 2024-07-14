use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day5.txt").unwrap();

	let mut part1 = 0;
	let mut part2 = 0;

	for line in file.lines() {
		if nice_model1(line) {
			part1 += 1;
		}
		if nice_model2(line) {
			part2 += 1;
		}
	}

	println!("Day 5 part 1: {}", part1);
	println!("Day 5 part 2: {}", part2);
}

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn nice_model1(str: &str) -> bool {
	if str.contains("ab") || str.contains("cd") || str.contains("pq") || str.contains("xy") {
		return false;
	}

	if str.chars().filter(|c| VOWELS.contains(c)).count() < 3 {
		return false;
	}

	let chars = str.chars().collect::<Vec<_>>();
	chars.as_slice().windows(2).any(|x| x[0] == x[1])
}

fn nice_model2(str: &str) -> bool {
	let chars = str.chars().collect::<Vec<_>>();
	if !chars.as_slice().windows(3).any(|x| x[0] == x[2]) {
		return false;
	}

	for i in 0..chars.len()-1 {
		let x1 = chars[i];
		let x2 = chars[i+1];
		for j in i+2..chars.len()-1 {
			let y1 = chars[j];
			let y2 = chars[j+1];
			if x1 == y1 && x2 == y2 {
				return true;
			}
		}
	}
	
	false
}