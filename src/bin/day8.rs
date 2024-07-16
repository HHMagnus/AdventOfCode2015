use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day8.txt").unwrap();

	let part1 = file.lines().map(|x| x.len() - str_val(x)).sum::<usize>();
	println!("Day 8 part 1: {}", part1);
}

fn str_val(x: &str) -> usize {
	let mut chars = x.chars();
	let mut count = 0;
	while let Some(x) = chars.next() {
		if x == '\\' {
			let y = chars.next().unwrap();
			if y == 'x' {
				chars.next().unwrap();
				chars.next().unwrap();
			}
		}
		count += 1;
	}
	count - 2
}
