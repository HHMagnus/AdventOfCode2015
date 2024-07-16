use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day15.txt").unwrap();

	let lines = file.lines().map(|line| {
		let mut split = line.split(": capacity ");
		let name = split.next().unwrap();
		let mut split = split.next().unwrap().split(", durability ");
		let capacity = split.next().unwrap().parse::<i32>().unwrap();
		let mut split = split.next().unwrap().split(", flavor ");
		let durability = split.next().unwrap().parse::<i32>().unwrap();
		let mut split = split.next().unwrap().split(", texture ");
		let flavor = split.next().unwrap().parse::<i32>().unwrap();
		let mut split = split.next().unwrap().split(", calories ");
		let texture = split.next().unwrap().parse::<i32>().unwrap();
		let calories = split.next().unwrap().parse::<i32>().unwrap();
		(name, capacity, durability, flavor, texture, calories)
	}).collect::<Vec<_>>();

	let mut part1 = 0;
	let mut part2 = 0;
	for i in 0..=100 {
		for j in 0..=(100 - i) {
			for k in 0..=(100 - i - j) {
				let h = 100 - i - j - k;

				let capacity = (0..lines.len()).map(|x| lines[x].1).zip([i, j, k, h]).map(|x| x.0 * x.1).sum::<i32>();
				let durability = (0..lines.len()).map(|x| lines[x].2).zip([i, j, k, h]).map(|x| x.0 * x.1).sum::<i32>();
				let flavor = (0..lines.len()).map(|x| lines[x].3).zip([i, j, k, h]).map(|x| x.0 * x.1).sum::<i32>();
				let texture = (0..lines.len()).map(|x| lines[x].4).zip([i, j, k, h]).map(|x| x.0 * x.1).sum::<i32>();
				let calories = (0..lines.len()).map(|x| lines[x].5).zip([i, j, k, h]).map(|x| x.0 * x.1).sum::<i32>();

				let total = capacity.max(0) * durability.max(0) * flavor.max(0) * texture.max(0);

				part1 = part1.max(total);

				if calories == 500 {
					part2 = part2.max(total);
				}
			}
		}
	}
	println!("Day 15 part 1: {}", part1);
	println!("Day 15 part 2: {}", part2);
}