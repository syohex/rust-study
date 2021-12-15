use std::collections::HashSet;
use std::fs;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut bs: Vec<u8> = vec![];
            for b in s.bytes() {
                bs.push(b);
            }
            bs
        })
        .collect()
}

fn part1(data: &Vec<Vec<u8>>) -> i32 {
    let count = data.len();

    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..data[0].len() {
        let ones = data.iter().filter(|v| v[i] == b'1').count();
        let zeros = count - ones;

        if ones > zeros {
            gamma = gamma * 2 + 1;
            epsilon = epsilon * 2;
        } else {
            gamma = gamma * 2;
            epsilon = epsilon * 2 + 1;
        }
    }

    gamma * epsilon
}

fn part2(data: &Vec<Vec<u8>>) -> i32 {
    let size = data[0].len();
    let mut oxygen_set: HashSet<Vec<u8>> = HashSet::new();
    for d in data {
        oxygen_set.insert(d.clone());
    }

    let mut co2_set = oxygen_set.clone();

    let mut oxygen = 0;
    for i in 0..size {
        let ones = oxygen_set.iter().filter(|v| v[i] == b'1').count();
        let zeros = oxygen_set.len() - ones;
        let keep = if ones >= zeros { b'1' } else { b'0' };
        oxygen_set.retain(|v| v[i] == keep);

        if oxygen_set.len() == 1 {
            let remain = oxygen_set.iter().next().unwrap();
            oxygen = remain.iter().fold(0, |acc, b| acc * 2 + (*b - b'0') as i32);
            break;
        }
    }

    let mut co2 = 0;
    for i in 0..size {
        let ones = co2_set.iter().filter(|v| v[i] == b'1').count();
        let zeros = co2_set.len() - ones;
        let keep = if ones >= zeros { b'0' } else { b'1' };

        co2_set.retain(|v| v[i] == keep);

        if co2_set.len() == 1 {
            let remain = co2_set.iter().next().unwrap();
            co2 = remain.iter().fold(0, |acc, b| acc * 2 + (*b - b'0') as i32);
            break;
        }
    }

    oxygen * co2
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let data = parse_input(&input);
    let answer1 = part1(&data);
    let answer2 = part2(&data);

    println!("Part1: {}", answer1);
    println!("Part2: {}", answer2);

    assert_eq!(answer1, 3912944);
    assert_eq!(answer2, 4996233);
}

#[test]
fn test() {
    let input = r#"
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
"#;

    let data = parse_input(input);
    let answer1 = part1(&data);
    let answer2 = part2(&data);

    assert_eq!(answer1, 198);
    assert_eq!(answer2, 230);
}
