use std::fs;
use std::io::{self, BufRead};

fn count_increased(nums: &Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut count = 0;
    let mut prev = nums[0];
    for &num in nums.iter().skip(1) {
        if num > prev {
            count += 1
        }

        prev = num;
    }

    count
}

fn count_increased_three_pair(nums: &Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut count = 0;
    let mut prev = nums[0] + nums[1] + nums[2];
    for i in 3..nums.len() {
        let sum = nums[i] + nums[i - 1] + nums[i - 2];
        if sum > prev {
            count += 1;
        }

        prev = sum;
    }

    count
}

fn main() {
    let file = fs::File::open("data/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let nums = lines
        .filter_map(|line| {
            if let Ok(num_str) = line {
                num_str.parse::<i32>().ok()
            } else {
                None
            }
        })
        .collect::<Vec<i32>>();

    let part1 = count_increased(&nums);
    let part2 = count_increased_three_pair(&nums);
    assert_eq!(part1, 1448);
    assert_eq!(part2, 1471);
    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
}

#[test]
fn test_count_increased() {
    let nums = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(count_increased(&nums), 7);
    assert_eq!(count_increased_three_pair(&nums), 5);
}
