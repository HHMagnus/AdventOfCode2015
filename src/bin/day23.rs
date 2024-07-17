use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Hlf(char),
    Tpl(char),
    Inc(char),
    Jmp(i64),
    Jie(char, i64),
    Jio(char, i64),
}

fn main() {
    let file = read_to_string("input/day23.txt").unwrap();

    let instructions = file
        .lines()
        .map(|line| {
            if line.contains("hlf") {
                Instruction::Hlf(line.chars().nth(4).unwrap())
            } else if line.contains("tpl") {
                Instruction::Tpl(line.chars().nth(4).unwrap())
            } else if line.contains("inc") {
                Instruction::Inc(line.chars().nth(4).unwrap())
            } else if line.contains("jmp") {
                Instruction::Jmp(line.replace("jmp ", "").parse().unwrap())
            } else if line.contains("jie") {
                let mut split = line.split(", ");
                Instruction::Jie(
                    split.next().unwrap().chars().nth(4).unwrap(),
                    split.next().unwrap().parse().unwrap(),
                )
            } else if line.contains("jio") {
                let mut split = line.split(", ");
                Instruction::Jio(
                    split.next().unwrap().chars().nth(4).unwrap(),
                    split.next().unwrap().parse().unwrap(),
                )
            } else {
                unreachable!("Unknown line: '{}'", line);
            }
        })
        .collect::<Vec<_>>();

    let part1 = solve(&instructions, 0);
    println!("Day 23 part 1: {}", part1);

    let part2 = solve(&instructions, 1);
    println!("Day 23 part 2: {}", part2);
}

fn solve(instructions: &Vec<Instruction>, a: i64) -> i64 {
    let mut registers = HashMap::new();
    registers.insert('a', a);
    registers.insert('b', 0);

    let mut i = 0;
    while i >= 0 && i < instructions.len() as i64 {
        let inst = instructions[i as usize];
        match inst {
            Instruction::Hlf(r) => {
                *registers.get_mut(&r).unwrap() /= 2;
            }
            Instruction::Tpl(r) => {
                *registers.get_mut(&r).unwrap() *= 3;
            }
            Instruction::Inc(r) => {
                *registers.get_mut(&r).unwrap() += 1;
            }
            Instruction::Jmp(offset) => {
                i += offset;
                continue;
            }
            Instruction::Jie(r, offset) => {
                if registers[&r] % 2 == 0 {
                    i += offset;
                    continue;
                }
            }
            Instruction::Jio(r, offset) => {
                if registers[&r] == 1 {
                    i += offset;
                    continue;
                }
            }
        }
        i += 1;
    }

    registers[&'b']
}
