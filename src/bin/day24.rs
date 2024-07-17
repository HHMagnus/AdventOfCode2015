use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let file = read_to_string("input/day24.txt").unwrap();

	let input = file.lines().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

	let sum = input.iter().sum::<usize>() / 3;

	let mut vec = input.iter().map(|&x| vec![x]).collect::<HashSet<_>>();

	while !vec.iter().any(|x| x.iter().sum::<usize>() == sum) {
		vec = vec.into_iter().flat_map(|x| {
			let mut new = Vec::new();
			for &y in &input {
				if x.contains(&y) { continue; }
				let mut n = x.clone();
				n.push(y);
				n.sort();
				new.push(n);
			}
			new
		}).filter(|x| x.iter().sum::<usize>() <= sum).collect();
	}

	let sums = vec.into_iter().filter(|x| x.iter().sum::<usize>() == sum).collect::<Vec<_>>();
	
	let part1 = sums.into_iter().map(|x| x.into_iter().fold(1, |acc, y| acc * y)).min().unwrap();
	println!("Day 24 part 1: {}", part1);
}