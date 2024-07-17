use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day20.txt").unwrap();

	let input = file.parse::<usize>().unwrap();

	let mut part1 = 1;

	while present1(part1) < input {
		part1 += 1;
	}

	println!("Day 20 part 1: {}", part1);

	let mut part2 = 1;

	while present2(part2) < input {
		part2 += 1;
	}

	println!("Day 20 part 2: {}", part2);
}

fn present1(i: usize) -> usize {
	if i == 1 {
		return 1;
	}
	let mut total = 0;

	for x in 2..=(i as f32).sqrt() as usize {
		if i % x == 0 {
			if x == (i / x) {
				total += x;
			} else {
				total += x + i/x;
			}
		}
	}
	// Sum of divisors * 10
	(total + i + 1) * 10
}

fn present2(i: usize) -> usize {
	if i == 1 {
		return 1;
	}
	let mut total = 0;

	for x in 2..=(i as f32).sqrt() as usize {
		if i % x == 0 {
			let factor = x;
			if i / factor < 50 {
				total += factor
			}
			let factor = i / x;
			if x != factor && i / factor < 50 {
				total += factor
			}
		}
	}
	(total + i + 1) * 11
}