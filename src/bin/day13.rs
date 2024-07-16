use std::{collections::{HashMap, HashSet}, fs::read_to_string};
use itertools::Itertools;

fn main() {
	let file = read_to_string("input/day13.txt").unwrap();

	let mut input = file.lines().map(|line| {
		let mut split = line.split(" happiness units by sitting next to ");
		let f = split.next().unwrap();
		let next = split.next().unwrap().replace(".", "");
		let mut split = f.split(" would ");
		let first = split.next().unwrap();
		let amount = split.next().unwrap().replace("gain ", "").replace("lose ", "-").parse::<i32>().unwrap();
		((first.to_string(), next), amount)
	}).collect::<HashMap<_, _>>();

	let available = input.iter().map(|x| x.0.0.clone()).collect::<HashSet<_>>();
	let mut available = available.into_iter().collect::<Vec<_>>();
	let total = available.len();

	let part1 = available.clone().into_iter().permutations(total).map(|perm| roundness(&input, &perm)).max().unwrap();
	println!("Day 13 part 1: {}", part1);

	for x in available.clone() {
		input.insert(("Me".to_string(), x.clone()), 0);
		input.insert((x.clone(), "Me".to_string()), 0);
	}

	available.push("Me".to_string());
	let total = available.len();
	
	let part2 = available.clone().into_iter().permutations(total).map(|perm| roundness(&input, &perm)).max().unwrap();
	println!("Day 13 part 2: {}", part2);
}

fn roundness(map: &HashMap<(String, String), i32>, values: &Vec<String>) -> i32 {
	let len = values.len();
	let mut val = 0;
	for i in 0..values.len() {
		let x1 = values[i].clone();
		let x2 = values[(i + 1) % len].clone();
		val += map[&(x1.clone(), x2.clone())];
		val += map[&(x2, x1)];
	}
	val
}