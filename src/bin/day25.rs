use std::fs::read_to_string;

fn main() {
    let file = read_to_string("input/day25.txt").unwrap();

	let mut split = file.split("row ");
	split.next();
	let mut split = split.next().unwrap().split(", column ");
	let row = split.next().unwrap().parse::<usize>().unwrap();
	let column = split.next().unwrap().replace(".\n", "").parse::<usize>().unwrap();

	let mut curr = 20151125i128;
	let mul = 252533i128;
	let remain = 33554393i128;

	let mut y = 1;
	let mut pos = (1, 1);
	loop {
		pos = (pos.0 + 1, pos.1 - 1);
		if pos.1 < 1 {
			y += 1;
			pos = (1, y);
		}

		curr = (curr * mul) % remain;
		
		if pos.0 == column && pos.1 == row {
			println!("Day 25: {}", curr);
			break;
		}
	}
}