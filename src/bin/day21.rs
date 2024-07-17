use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day21.txt").unwrap();

	let mut lines =  file.lines().map(|x| {
		let mut x = x.split(": ");
		x.next();
		x.next().unwrap().parse::<i32>().unwrap()
	});
	let boss = (lines.next().unwrap(), lines.next().unwrap(), lines.next().unwrap());

	let weapons = [
		(8, 4, 0),
		(10, 5, 0),
		(25, 6, 0),
		(40, 7, 0),
		(74, 8, 0)
	];

	let armor = [
		(13, 0, 1),
		(31, 0, 2),
		(53, 0, 3),
		(75, 0, 4),
		(102, 0, 5),
	];

	let rings = [
		(25, 1, 0),
		(50, 2, 0),
		(100, 3, 0),
		(20, 0, 1),
		(40, 0, 2),
		(80, 0, 3)
	];

	let mut equipment = weapons.to_vec();

	for x in equipment.clone().into_iter() {
		for a in armor {
			equipment.push((x.0+a.0,x.1+a.1,x.2+a.2));
		}
	}

	let armor_and_weapons = equipment.clone();

	for x in armor_and_weapons {
		for r in rings {
			equipment.push((x.0+r.0,x.1+r.1,x.2+r.2));
		}

		for i in 0..rings.len() {
			for j in i+1..rings.len() {
				let r1 = rings[i];
				let r2 = rings[j];
				equipment.push((x.0+r1.0+r2.0, x.1+r1.1+r2.1, x.2+r1.2+r2.2));
			}
		}
	}

	let part1 = equipment.clone().into_iter().filter(|x| fight(boss, (100, x.1, x.2))).min_by_key(|x| x.0).unwrap().0;
	println!("Day 21 part 1: {}", part1);

	let part2 = equipment.into_iter().filter(|x| !fight(boss, (100, x.1, x.2))).max_by_key(|x| x.0).unwrap().0;
	println!("Day 21 part 2: {}", part2);
}

fn fight(mut boss: (i32, i32, i32), mut player: (i32, i32, i32)) -> bool {
	loop {
		boss.0 -= (player.1 - boss.2).max(1);
		if boss.0 <= 0 {
			return true;
		}
		player.0 -= (boss.1 - player.2).max(1);
		if player.0 <= 0 {
			return false;
		}
	}
}