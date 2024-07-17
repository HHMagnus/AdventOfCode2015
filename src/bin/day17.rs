use std::{collections::HashMap, fs::read_to_string};

fn main() {
	let file = read_to_string("input/day17.txt").unwrap();

	let input = file.lines().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

	let mut part1 = 0;
	let mut part2 = i32::MAX;
	for i in 0..(1 << input.len()) {
		let mut total = 0;
		let mut i = i;
		let mut x = 0;
		let mut count = 0;
		while i > 0 {
			if i % 2 == 1 {
				total += input[x];
				count += 1;
			}
			x+=1;
			i >>= 1;
		}
		if total == 150 {
			part1 += 1;
			part2 = part2.min(count);
		}
	}

	println!("Day 17 part 1: {}", part1);
	println!("Day 17 part 2: {}", part2);
}