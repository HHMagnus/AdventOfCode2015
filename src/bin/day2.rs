use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day2.txt").unwrap();

	let input = file.lines().map(|line| {
		let mut split = line.split("x");
		let x = split.next().unwrap().parse::<usize>().unwrap();
		let y = split.next().unwrap().parse::<usize>().unwrap();
		let z = split.next().unwrap().parse::<usize>().unwrap();
		(x, y, z)
	}).collect::<Vec<_>>();

	let part1 = input.iter().cloned().map(square_feet_of_wrapping_paper).sum::<usize>();
	println!("Day 2 part 1: {}", part1);
	let part2 = input.into_iter().map(ribbon).sum::<usize>();
	println!("Day 2 part 2: {}", part2);
}

fn square_feet_of_wrapping_paper(x: (usize, usize, usize)) -> usize {
	let side1 = x.0*x.1;
	let side2 = x.1*x.2;
	let side3 = x.2*x.0;
	side1.min(side2).min(side3) + 2*side1 + 2*side2 + 2*side3
}

fn ribbon(x: (usize, usize, usize)) -> usize {
	let ribbon = x.0*x.1*x.2;

	let length = if x.0 >= x.1 && x.0 >= x.2 {
		x.1*2+x.2*2
	} else if x.1 >= x.2 && x.1 >= x.0 {
		x.0*2+x.2*2
	} else {
		x.0*2+x.1*2
	};

	ribbon + length
}