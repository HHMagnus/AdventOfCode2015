use std::{collections::{HashSet, VecDeque}, fs::read_to_string};

fn main() {
	let file = read_to_string("input/day9.txt").unwrap();

	let input = file.lines().map(|line| {
		let mut split = line.split(" = ");
		let mut cities = split.next().unwrap().split(" to ");
		let city1 = cities.next().unwrap();
		let city2 = cities.next().unwrap();
		let num = split.next().unwrap().parse::<usize>().unwrap();
		(city1, city2, num)
	}).collect::<Vec<_>>();

	let queue = input.iter().cloned().flat_map(|x| [(x.0, vec![x.0], 0), (x.1, vec![x.1], 0)]).collect::<HashSet<_>>();
	let mut queue = queue.into_iter().collect::<VecDeque<_>>();
	let total = queue.len();

	let mut part1 = usize::MAX;
	let mut part2 = usize::MIN;

	while let Some((curr, next, value)) = queue.pop_front() {
		if next.len() == total {
			part1 = part1.min(value);
			part2 = part2.max(value);
		}

		for x in input.iter().filter(|&&x| x.0 == curr && !next.contains(&x.1)) {
			let mut nexts = next.clone();
			nexts.push(x.1);
			queue.push_front((x.1, nexts, value + x.2));
		}

		for x in input.iter().filter(|&&x| x.1 == curr && !next.contains(&x.0)) {
			let mut nexts = next.clone();
			nexts.push(x.0);
			queue.push_front((x.0, nexts, value + x.2));
		}
	}

	println!("Day 9 part 1: {}", part1);
	println!("Day 9 part 2: {}", part2);
}