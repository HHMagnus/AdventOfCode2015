use std::{collections::HashSet, fs::read_to_string};

fn main() {
	let file = read_to_string("input/day3.txt").unwrap();

	let mut santa = (0, 0);
	let mut visited = HashSet::new();
	visited.insert(santa);

	for c in file.chars() {
		match c {
			'^' => santa = (santa.0, santa.1+1),
			'v' => santa = (santa.0, santa.1-1),
			'<' => santa = (santa.0-1, santa.1),
			'>' => santa = (santa.0+1, santa.1),
			x => unreachable!("Unknown: {}", x),
		}
		visited.insert(santa);
	}

	println!("Day 3 part 1: {}", visited.len());

	let mut santa = (0, 0);
	let mut robo = (0, 0);
	let mut turn = false;
	let mut visited = HashSet::new();
	visited.insert(santa);

	for c in file.chars() {
		let pos = if turn { santa } else { robo };
		let newpos = match c {
			'^' => (pos.0, pos.1+1),
			'v' => (pos.0, pos.1-1),
			'<' => (pos.0-1, pos.1),
			'>' => (pos.0+1, pos.1),
			x => unreachable!("Unknown: {}", x),
		};
		if turn {
			santa = newpos;
		} else {
			robo = newpos;
		}
		visited.insert(santa);
		visited.insert(robo);
		turn = !turn;
	}

	println!("Day 3 part 2: {}", visited.len());
}