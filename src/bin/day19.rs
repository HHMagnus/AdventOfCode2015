use std::{collections::HashSet, fs::read_to_string};

fn main() {
	let file = read_to_string("input/day19.txt").unwrap();

	let mut split = file.split("\n\n");
	
	let transforms = split.next().unwrap().lines().map(|line| {
		let mut split = line.split(" => ");
		(split.next().unwrap(), split.next().unwrap())
	}).collect::<Vec<_>>();

	let medicine = split.next().unwrap();

	let part1 = transforms.into_iter().flat_map(|x| apply(medicine, x)).collect::<HashSet<_>>();

	println!("Day 19 part 1: {}", part1.len());
}

fn apply(medicine: &str, transform: (&str, &str)) -> Vec<String> {
	let mut result = Vec::new();
	for (i, _) in medicine.match_indices(transform.0) {
		let mut new = String::new();
		new.push_str(&medicine[..i]);
		new.push_str(transform.1);
		let id = i+transform.0.len();
		if id <= medicine.len() {
			new.push_str(&medicine[id..]);
		}
		result.push(new);
	}
	result
}