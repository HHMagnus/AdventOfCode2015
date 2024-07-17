use std::{collections::{HashSet, VecDeque}, fs::read_to_string};

fn main() {
	let file = read_to_string("input/day19.txt").unwrap();

	let mut split = file.split("\n\n");
	
	let transforms = split.next().unwrap().lines().map(|line| {
		let mut split = line.split(" => ");
		(split.next().unwrap(), split.next().unwrap())
	}).collect::<Vec<_>>();

	let medicine = split.next().unwrap().lines().next().unwrap();

	let part1 = transforms.iter().flat_map(|&x| apply(medicine, x)).collect::<HashSet<_>>();

	println!("Day 19 part 1: {}", part1.len());

	let mut queue = VecDeque::new();
	queue.push_back((medicine.to_string(), 1));
	let mut visited = HashSet::new();
	visited.insert(medicine.to_string());

	'bfs: while let Some((str, steps)) = queue.pop_front() {

		let x = transforms.iter().flat_map(|&(t1, t2)| apply(&str, (t2, t1))).min_by_key(|x| x.len()).unwrap();
				if visited.contains(&x) {
					continue;
				}
				visited.insert(x.clone());

				if x == "e" {
					println!("Day 19 part 2: {}", steps);
					break 'bfs;
				}

				queue.push_front((x, steps + 1));
	}
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