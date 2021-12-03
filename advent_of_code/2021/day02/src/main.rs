use std::fs::File;
use std::io::{BufRead, BufReader};

enum Direction {
    Forward,
    Down,
    Up,
}

struct Command {
    direction: Direction,
    count: i32,
}

fn calculate_position(commands: &Vec<Command>) -> i32 {
    let mut x = 0;
    let mut y = 0;

    for command in commands {
        match command.direction {
            Direction::Forward => {
                x += command.count;
            }
            Direction::Down => {
                y += command.count;
            }
            Direction::Up => {
                y -= command.count;
            }
        }
    }

    x * y
}

fn calculate_position_with_aim(commands: &Vec<Command>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;

    for command in commands {
        match command.direction {
            Direction::Forward => {
                x += command.count;
                y += aim * command.count;
            }
            Direction::Down => {
                aim += command.count;
            }
            Direction::Up => {
                aim -= command.count;
            }
        }
    }

    x * y
}
fn main() {
    let file = File::open("data/input.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let commands: Vec<Command> = lines
        .filter_map(|res| {
            if let Ok(line) = res {
                let parts: Vec<&str> = line.split(" ").collect();
                let direction = match parts[0] {
                    "forward" => Direction::Forward,
                    "down" => Direction::Down,
                    "up" => Direction::Up,
                    _ => panic!("unknown direction"),
                };
                let count = parts[1].parse::<i32>().unwrap();
                Some(Command { direction, count })
            } else {
                None
            }
        })
        .collect();

    let part1 = calculate_position(&commands);
    let part2 = calculate_position_with_aim(&commands);
    assert_eq!(part1, 1868935);
    assert_eq!(part2, 1965970888);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
