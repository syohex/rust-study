fn add_only_even(nums: Vec<i32>) -> Option<i32> {
    nums.iter()
        .try_fold(0, |acc, n| {
            if n % 2 == 0 {
                Ok(acc + n)
            } else {
                Err("Invalid")
            }
        })
        .ok()
}

fn main() {
    println!("add_only_even([2, 4])={:?}", add_only_even(vec![2, 4]));
}

#[test]
fn test_add_only_even() {
    assert_eq!(add_only_even(vec![2, 4, 6]), Some(12));
    assert_eq!(add_only_even(vec![1, 2, 3]), None);
}
