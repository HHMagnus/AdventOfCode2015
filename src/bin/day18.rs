use std::{collections::HashSet, fs::read_to_string};

fn main() {
	let file = read_to_string("input/day18.txt").unwrap();

	let map = file.lines().enumerate().flat_map(|(y, cs)| cs.chars().enumerate().filter_map(move |(x, c)| if c == '#' { Some((x as i32, y as i32)) } else { None })).collect::<HashSet<_>>();

	let mut part1 = map.clone();
	for _ in 0..100 {
		let mut replacement = HashSet::new();
		for x in 0..100 {
			for y in 0..100 {
				let neighs = [
					(x - 1, y - 1),
					(x - 1, y),
					(x - 1, y + 1),
					(x, y - 1),
					(x, y + 1),
					(x + 1, y - 1),
					(x + 1, y),
					(x + 1, y + 1)
				].into_iter().filter(|x| part1.contains(x)).count();
				let curr = part1.contains(&(x, y));

				if curr && (neighs == 2 || neighs == 3) {
					replacement.insert((x, y));
				} else if !curr && neighs == 3 {
					replacement.insert((x, y));
				}

			}
		}
		part1 = replacement;
	}

	println!("Day 18 part 1: {}", part1.len());

	let maxx = *map.iter().map(|(x, _)| x).max().unwrap();
	let maxy = *map.iter().map(|(_, y)| y).max().unwrap();

	let mut part2 = map.clone();
	part2.insert((0,0));
	part2.insert((maxx,0));
	part2.insert((maxx,maxy));
	part2.insert((0,maxy));
	for _ in 0..100 {
		let mut replacement = HashSet::new();
		for x in 0..100 {
			for y in 0..100 {
				let neighs = [
					(x - 1, y - 1),
					(x - 1, y),
					(x - 1, y + 1),
					(x, y - 1),
					(x, y + 1),
					(x + 1, y - 1),
					(x + 1, y),
					(x + 1, y + 1)
				].into_iter().filter(|x| part2.contains(x)).count();
				let curr = part2.contains(&(x, y));

				if curr && (neighs == 2 || neighs == 3) {
					replacement.insert((x, y));
				} else if !curr && neighs == 3 {
					replacement.insert((x, y));
				}

			}
		}
		part2 = replacement;
		part2.insert((0,0));
		part2.insert((maxx,0));
		part2.insert((maxx,maxy));
		part2.insert((0,maxy));
	}

	println!("Day 18 part 2: {}", part2.len());
}