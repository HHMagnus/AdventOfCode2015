use std::{collections::{HashMap, HashSet}, fs::read_to_string};

#[derive(Debug, Clone, Copy)]
enum Toggle {
	Toggle((usize, usize), (usize, usize)),
	TurnOn((usize, usize), (usize, usize)),
	TurnOff((usize, usize), (usize, usize)),
}

fn main() {
	let file = read_to_string("input/day6.txt").unwrap();

	let instructions = file.lines().map(|line| {
		let mut split = line.split(" through ");
		let x = split.next().unwrap().replace("toggle ", "").replace("turn off ", "").replace("turn on ", "");
		let y = split.next().unwrap();
		let mut xsplit = x.split(",");
		let mut ysplit = y.split(",");
		let x = (xsplit.next().unwrap().parse::<usize>().unwrap(), xsplit.next().unwrap().parse::<usize>().unwrap());
		let y = (ysplit.next().unwrap().parse::<usize>().unwrap(), ysplit.next().unwrap().parse::<usize>().unwrap());
		if line.contains("turn on") {
			Toggle::TurnOn(x, y)
		} else if line.contains("turn off") {
			Toggle::TurnOff(x, y)
		} else if line.contains("toggle") {
			Toggle::Toggle(x, y)
		} else {
			unreachable!("Unknown: '{}'", line);
		}
	}).collect::<Vec<_>>();

	let mut turned = HashSet::new();

	for &instruction in &instructions {
		match instruction {
			Toggle::Toggle(x0, y0) => {
				for y in x0.0..=y0.0 {
					for x in x0.1..=y0.1 {
						let key = (x, y);
						if turned.contains(&key) {
							turned.remove(&key);
						} else {
							turned.insert(key);
						}
					}
				}
			},
			Toggle::TurnOn(x0, y0) => {
				for y in x0.0..=y0.0 {
					for x in x0.1..=y0.1 {
						let key = (x, y);
						turned.insert(key);
					}
				}
			},
			Toggle::TurnOff(x0, y0) => {
				for y in x0.0..=y0.0 {
					for x in x0.1..=y0.1 {
						let key = (x, y);
						turned.remove(&key);
					}
				}
			},
		}
	}

	println!("Day 6 part 1: {}", turned.len());

	let mut turned = HashMap::new();

	for x in 0..1000 {
		for y in 0..1000 {
			turned.insert((x, y), 0);
		}
	}

	for instruction in instructions {
		match instruction {
			Toggle::Toggle(x0, y0) => {
				for y in x0.0..=y0.0 {
					for x in x0.1..=y0.1 {
						let key = (x, y);
						*turned.get_mut(&key).unwrap() += 2;
					}
				}
			},
			Toggle::TurnOn(x0, y0) => {
				for y in x0.0..=y0.0 {
					for x in x0.1..=y0.1 {
						let key = (x, y);
						*turned.get_mut(&key).unwrap() += 1;
					}
				}
			},
			Toggle::TurnOff(x0, y0) => {
				for y in x0.0..=y0.0 {
					for x in x0.1..=y0.1 {
						let key = (x, y);
						if turned[&key] > 0 {
							*turned.get_mut(&key).unwrap() -= 1;
						}
					}
				}
			},
		}
	}

	let part2 = turned.values().sum::<usize>();
	println!("Day 6 part 2: {}", part2);
}