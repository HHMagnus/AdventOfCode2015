use std::{cmp::Ordering, collections::BinaryHeap, fs::read_to_string};

#[derive(Debug, Clone, Eq, PartialEq)]
enum Difficulty {
	Normal,
	Hard,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Fight {
	boss_hit_points: i32,
	boss_damage: i32,
	player_hit_points: i32,
	player_mana: i32,
	shield: Option<usize>,
	poison: Option<usize>,
	recharge: Option<usize>,
	mana_spent: i32,
	player_turn: bool,
	difficulty: Difficulty,
}

impl Ord for Fight {
    fn cmp(&self, other: &Self) -> Ordering {
        other.mana_spent.cmp(&self.mana_spent)
    }
}

impl PartialOrd for Fight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Fight {
	fn init(player_hit_points: i32, player_mana: i32, boss_hit_points: i32, boss_damage: i32, difficulty: Difficulty) -> Fight {
		Fight {
			boss_hit_points,
			boss_damage,
			player_hit_points,
			player_mana,
			shield: None,
			poison: None,
			recharge: None,
			mana_spent: 0,
			player_turn: true,
			difficulty,
		}
	}

	fn spent_mana(&mut self, amount: i32) {
		self.mana_spent += amount;
		self.player_mana -= amount;
	}

	fn turn(&self) -> Vec<Fight> {
		if !self.player_turn {
			let mut boss = self.clone();
			boss.turn_boss();
			boss.player_turn = true;
			return vec![boss];
		}
		self.turn_player()
	}

	fn turn_player(&self) -> Vec<Fight> {
		let mut fight = self.clone();

		if self.difficulty == Difficulty::Hard {
			fight.player_hit_points -= 1;
			if fight.boss_won() {
				return vec![fight];
			}
		}

		fight.effects();
		fight.player_turn = false;

		let mut result = Vec::new();
		if fight.player_mana >= 53 {
			let mut missiles = fight.clone();
			missiles.spent_mana(53);
			missiles.boss_hit_points -= 4;
			result.push(missiles);
		}
		if fight.player_mana >= 73 {
			let mut drain = fight.clone();
			drain.spent_mana(73);
			drain.boss_hit_points -= 2;
			drain.player_hit_points += 2;
			result.push(drain);
		}
		if fight.player_mana >= 113 && fight.shield.is_none() {
			let mut shield = fight.clone();
			shield.spent_mana(113);
			shield.shield = Some(6);
			result.push(shield);
		}
		if fight.player_mana >= 173 && fight.poison.is_none() {
			let mut poison = fight.clone();
			poison.spent_mana(173);
			poison.poison = Some(6);
			result.push(poison);
		}
		if fight.player_mana >= 229 && fight.recharge.is_none() {
			let mut recharge = fight.clone();
			recharge.spent_mana(229);
			recharge.recharge = Some(5);
			result.push(recharge);
		}
		result
	}

	fn turn_boss(&mut self) {
		self.effects();

		if self.player_won() {
			return;
		}
		
		let damage = (self.boss_damage - if self.shield.is_some() { 7 } else { 0 }).max(1);
		self.player_hit_points -= damage;
	}

	fn effects(&mut self) {
		if let Some(shield) = self.shield {
			let shield = shield - 1;
			if shield == 0 {
				self.shield = None;
			} else {
				self.shield = Some(shield);
			}
		}

		if let Some(poison) = self.poison {
			let poison = poison - 1;
			if poison == 0 {
				self.poison = None;
			} else {
				self.poison = Some(poison);
			}
			self.boss_hit_points -= 3;
		}

		if let Some(recharge) = self.recharge {
			let recharge = recharge - 1;
			if recharge == 0 {
				self.recharge = None;
			} else {
				self.recharge = Some(recharge);
			}
			self.player_mana += 101;
		}
	}

	fn player_won(&self) -> bool {
		self.boss_hit_points <= 0
	}

	fn boss_won(&self) -> bool {
		self.player_hit_points <= 0
	}
}

fn main() {
	let file = read_to_string("input/day22.txt").unwrap();

	let mut lines =  file.lines().map(|x| {
		let mut x = x.split(": ");
		x.next();
		x.next().unwrap().parse::<i32>().unwrap()
	});
	let boss_health = lines.next().unwrap();
	let boss_damage = lines.next().unwrap();

	//let fight = Fight::init(10, 250, 14, 8);
	let fight1 = Fight::init(50, 500, boss_health, boss_damage, Difficulty::Normal);
	let part1 = solve(fight1);
	println!("Day 22 part 1: {}", part1);
	
	let fight2 = Fight::init(50, 500, boss_health, boss_damage, Difficulty::Hard);
	let part2 = solve(fight2);
	println!("Day 22 part 2: {}", part2);
	
}

fn solve(initial: Fight) -> i32 {
	let mut queue = BinaryHeap::new();
	queue.push(initial);

	while let Some(fight) = queue.pop() {
		if fight.boss_won() {
			continue;
		}
		if fight.player_won() {
			return fight.mana_spent;
		}
		for x in fight.turn() {
			queue.push(x);
		}
	}

	unreachable!("No solution!");
}