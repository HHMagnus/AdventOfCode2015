use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day20.txt").unwrap();

	let input = file.parse::<usize>().unwrap();

	let mut i = 1;

	while present(i) < input {
		i += 1;
	}

	println!("Day 20 part 1: {}", i);
}

fn present(i: usize) -> usize {
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
	(total + i + 1) * 10
}