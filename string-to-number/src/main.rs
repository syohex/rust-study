fn main() {
    let input = r#"1
2
3
4
5
"#;

    let nums: Vec<i32> = input.lines().filter_map(|x| x.parse().ok()).collect();
    let sum: i32 = nums.iter().sum();
    println!("sum = {}", sum);
}
