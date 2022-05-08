fn sum_with_missing(numbers: Vec<Option<i32>>) -> i32 {
    numbers.into_iter().fold(0, |acc, n| match n {
        Some(m) => acc + m,
        None => acc + 0,
    })
}

fn main() {
    let nn = vec![None, Some(1), Some(5), Some(4), None, None];
    let ret = sum_with_missing(nn);
    println!("ret={ret}");
}

#[test]
fn empty() {
    let nn = vec![];
    assert_eq!(sum_with_missing(nn), 0);
}

#[test]
fn no_missing() {
    let nn = vec![Some(1), Some(5), Some(4)];
    assert_eq!(sum_with_missing(nn), 10);
}

#[test]
fn some_missing() {
    let nn = vec![None, Some(1), Some(5), Some(4), None, None];
    assert_eq!(sum_with_missing(nn), 10);
}

#[test]
fn all_missing() {
    let nn = vec![None, None, None];
    assert_eq!(sum_with_missing(nn), 0);
}
