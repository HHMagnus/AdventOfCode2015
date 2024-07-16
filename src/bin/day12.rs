use std::fs::read_to_string;

fn main() {
	let file = read_to_string("input/day12.json").unwrap();
    let json: serde_json::Value = serde_json::from_str(&file).expect("JSON was not well-formatted");
	let part1 = solve1(json.clone());
	println!("Day 12 part 1: {}", part1);

	let part2 = solve2(json).unwrap();
	println!("Day 12 part 2: {}", part2);
}

fn solve1(json: serde_json::Value) -> i64 {
	match json {
		serde_json::Value::Null => unreachable!("Unknown null"),
		serde_json::Value::Bool(x) => unreachable!("Unknown {}", x),
		serde_json::Value::Number(x) => x.as_i64().unwrap(),
		serde_json::Value::String(_) => 0,
		serde_json::Value::Array(x) => x.into_iter().map(solve1).sum(),
		serde_json::Value::Object(x) => x.into_iter().map(|x| x.1).map(solve1).sum(),
	}
}

fn solve2(json: serde_json::Value) -> Option<i64> {
	match json {
		serde_json::Value::Null => unreachable!("Unknown null"),
		serde_json::Value::Bool(x) => unreachable!("Unknown {}", x),
		serde_json::Value::Number(x) => Some(x.as_i64().unwrap()),
		serde_json::Value::String(x) => if x == "red" { None } else { Some(0) },
		serde_json::Value::Array(x) => Some(x.into_iter().filter_map(solve2).sum()),
		serde_json::Value::Object(x) => {
			let arr = x.into_iter().map(|x| x.1).map(solve2).collect::<Vec<_>>();
			if arr.iter().any(Option::is_none) {
				Some(0)
			} else {
				Some(arr.into_iter().map(Option::unwrap).sum())
			}
		},
	}
}