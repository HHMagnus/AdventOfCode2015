use std::{collections::BTreeMap, fs::read_to_string};

#[derive(Debug)]
enum Input {
	Wire(String),
	Val(usize),
}

#[derive(Debug)]
enum Instruction {
	Signal(Input, String),
	And(Input, Input, String),
	Lshift(Input, Input, String),
	Or(Input, Input, String),
	Rshift(Input, Input, String),
	Not(Input, String)
}

fn main() {
	let file = read_to_string("input/day7.txt").unwrap();

	let instructions =  file.lines().map(|line| {
		let mut split = line.split(" -> ");
		let first = split.next().unwrap();
		let second = split.next().unwrap();
		if first.contains("NOT") {
			let x = first.replace("NOT ", "");
			Instruction::Not(input(&x), second.to_string())
		} else if first.contains("AND") {
			let mut split = first.split(" AND ");
			let x = split.next().unwrap();
			let y = split.next().unwrap();
			Instruction::And(input(x), input(y), second.to_string())
		} else if first.contains("LSHIFT") {
			let mut split = first.split(" LSHIFT ");
			let x = split.next().unwrap();
			let y = split.next().unwrap();
			Instruction::Lshift(input(x), input(y), second.to_string())
		} else if first.contains("OR") {
			let mut split = first.split(" OR ");
			let x = split.next().unwrap();
			let y = split.next().unwrap();
			Instruction::Or(input(x), input(y), second.to_string())
		} else if first.contains("RSHIFT") {
			let mut split = first.split(" RSHIFT ");
			let x = split.next().unwrap();
			let y = split.next().unwrap();
			Instruction::Rshift(input(x), input(y), second.to_string())
		} else {
			Instruction::Signal(input(first), second.to_string())
		}
	}).collect::<Vec<_>>();
	
	let part1 = solve(&instructions, None);
	println!("Day 7 part 1: {}", part1);

	let part2 = solve(&instructions, Some(part1));
	println!("Day 7 part 2: {}", part2);
}

fn solve(instructions: &Vec<Instruction>, a: Option<usize>) -> usize {
	let mut wires = BTreeMap::new();

	if a.is_some() {
		wires.insert("b".to_string(), a.unwrap());
	}
	while !wires.contains_key("a") {
			for inst in instructions {
				match inst {
					Instruction::Signal(x, y) => {
						if a.is_some() && y == "b" {
							continue;
						}
						let val = match x {
							Input::Wire(x) => {
								if !wires.contains_key(x) {
									continue;
								}
								wires[x]
							},
							Input::Val(x) => *x,
						};
						wires.insert(y.clone(), val);
					},
					Instruction::And(x, y, z) => {
						let val1 = match x {
							Input::Wire(x) => {
								if !wires.contains_key(x) {
									continue;
								}
								wires[x]
							},
							Input::Val(x) => *x,
						};
						let val2 = match y {
							Input::Wire(y) => {
								if !wires.contains_key(y) {
									continue;
								}
								wires[y]
							},
							Input::Val(y) => *y,
						};
						let val = val1 & val2;
						wires.insert(z.clone(), val);
					},
					Instruction::Lshift(x, y, z) => {
						let val1 = match x {
							Input::Wire(x) => {
								if !wires.contains_key(x) {
									continue;
								}
								wires[x]
							},
							Input::Val(x) => *x,
						};
						let val2 = match y {
							Input::Wire(y) => {
								if !wires.contains_key(y) {
									continue;
								}
								wires[y]
							},
							Input::Val(y) => *y,
						};
						let val = val1 << val2;
						wires.insert(z.clone(), val);
					},
					Instruction::Or(x, y, z) => {
						let val1 = match x {
							Input::Wire(x) => {
								if !wires.contains_key(x) {
									continue;
								}
								wires[x]
							},
							Input::Val(x) => *x,
						};
						let val2 = match y {
							Input::Wire(y) => {
								if !wires.contains_key(y) {
									continue;
								}
								wires[y]
							},
							Input::Val(y) => *y,
						};
						let val = val1 | val2;
						wires.insert(z.clone(), val);
					},
					Instruction::Rshift(x, y, z) => {
						let val1 = match x {
							Input::Wire(x) => {
								if !wires.contains_key(x) {
									continue;
								}
								wires[x]
							},
							Input::Val(x) => *x,
						};
						let val2 = match y {
							Input::Wire(y) => {
								if !wires.contains_key(y) {
									continue;
								}
								wires[y]
							},
							Input::Val(y) => *y,
						};
						let val = val1 >> val2;
						wires.insert(z.clone(), val);
					},
					Instruction::Not(x, y) => {
						let val = match x {
							Input::Wire(x) => {
								if !wires.contains_key(x) {
									continue;
								}
								wires[x]
							},
							Input::Val(x) => *x,
						};
						let val = !val;
						wires.insert(y.clone(), val);
					},
				}
			}
		}
	
	wires["a"]
}

fn input(str: &str) -> Input {
	str.parse().map(|x| Input::Val(x)).unwrap_or(Input::Wire(str.to_string()))
}