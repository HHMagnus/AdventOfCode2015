use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day14.txt").unwrap();

	let input = file.lines().map(|line| {
		let mut split = line.split(" can fly ");
		let name = split.next().unwrap();
		let mut split = split.next().unwrap().split(" km/s for ");
		let kms = split.next().unwrap().parse::<usize>().unwrap();
		let mut split = split.next().unwrap().split(" seconds, but then must rest for ");
		let seconds = split.next().unwrap().parse::<usize>().unwrap();
		let mut split = split.next().unwrap().split(" seconds");
		let rest = split.next().unwrap().parse::<usize>().unwrap();
		(name, kms, seconds, rest)
	}).collect::<Vec<_>>();
	let total = 2503;

	let dists = input.iter().map(|x| dist(x, total)).collect::<Vec<_>>();

	let part1 = dists.iter().map(|x| x.last().unwrap()).max().unwrap();
	println!("Day 14 part 1: {}", part1);

	let mut points = Vec::new();
	for _ in 0..dists.len() {
		points.push(0);
	}

	for second in 0..total {
		let max = dists.iter().map(|x| x[second]).max().unwrap();
		for i in dists.iter().map(|x| x[second]).enumerate().filter(|&(_, x)| x == max).map(|(i, _)| i) {
			points[i] += 1;
		}
	}

	let part2 = points.iter().max().unwrap();
	println!("Day 14 part 2: {}", part2);
}

fn dist(input: &(&str, usize, usize, usize), seconds: usize) -> Vec<usize> {
	let mut dists = Vec::new();
	let mut dist = 0;
	let mut sec = 0;
	while sec < seconds {
		for _ in 0..input.2 {
			if sec == seconds {
				break;
			}
			sec += 1;
			dist += input.1;
			dists.push(dist);
		}
		for _ in 0..input.3 {
			if sec == seconds {
				break;
			}
			sec += 1;
			dists.push(dist);
		}
	}

	dists
}