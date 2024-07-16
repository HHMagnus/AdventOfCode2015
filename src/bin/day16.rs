use std::{collections::HashMap, fs::read_to_string};

fn main() {
	let file = read_to_string("input/day16.txt").unwrap();

	let mut known = HashMap::new();
	known.insert("children", 3);
	known.insert("cats", 7);
	known.insert("samoyeds", 2);
	known.insert("pomeranians", 3);
	known.insert("akitas", 0);
	known.insert("vizslas", 0);
	known.insert("goldfish", 5);
	known.insert("trees", 3);
	known.insert("cars", 2);
	known.insert("perfumes", 1);

	let lines = file.lines().map(|line| {
		let mut split = line.split(", ");
		let f = split.next().unwrap();
		let mut vec = Vec::new();
		for x in split {
			let mut s = x.split(": ");
			let item = s.next().unwrap();
			let amount = s.next().unwrap().parse::<usize>().unwrap();
			vec.push((item, amount));
		}
		let mut split = f.split(": ");
		let name = split.next().unwrap();
		let item = split.next().unwrap();
		let amount = split.next().unwrap().parse::<usize>().unwrap();
		vec.push((item, amount));
		(name, vec)
	}).collect::<Vec<_>>();

	'sue: for x in &lines {
		for item in &x.1 {
			if known[item.0] != item.1 {
				continue 'sue;
			}
		}
		println!("Day 16 part 1: {}", x.0);
		break;
	}

	'sue: for x in &lines {
		for item in &x.1 {
			if item.0 == "cats" || item.0 == "trees" {
				if known[item.0] >= item.1 {
					continue 'sue;
				}
			} else if item.0 == "pomeranians" || item.0 == "goldfish" {
				if known[item.0] <= item.1 {
					continue 'sue;
				}
			} else {
				if known[item.0] != item.1 {
					continue 'sue;
				}
			}
		}
		println!("Day 16 part 2: {}", x.0);
	}
}