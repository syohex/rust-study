// Specification
//  - List is empty, return None
//  - List length is odd, return median value
//  - List length is even, return average of two median values
fn calculate_median(nums: Vec<f32>) -> Option<f32> {
    let mut nums = nums;
    nums.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    let len = nums.len();
    if len == 0 {
        None
    } else if len % 2 == 1 {
        Some(nums[len / 2])
    } else {
        let a = nums[len / 2 - 1];
        let b = nums[len / 2];
        Some((a + b) / 2.0)
    }
}

fn main() {
    let nums = vec![1.5, 3.0, 5.0, 8.8];
    let ret = calculate_median(nums);
    println!("ret={:?}", ret);
}

#[test]
fn test_calculate_median() {
    {
        let nums = vec![];
        let ret = calculate_median(nums);
        assert_eq!(ret, None);
    }
    {
        let nums = vec![1.0, 3.0, 5.0];
        let ret = calculate_median(nums);
        assert_eq!(ret, Some(3.0));
    }
    {
        let nums = vec![1.5, 3.0, 5.0, 8.8];
        let ret = calculate_median(nums);
        assert_eq!(ret, Some(4.0));
    }
}
