use std::collections::HashSet;
use std::hash::Hash;

fn find_unique_items<T: Ord + Hash + Copy>(items: Vec<T>) -> Vec<T> {
    let mut ret: Vec<T> = vec![];
    let mut s: HashSet<T> = HashSet::new();

    for item in &items {
        if !s.contains(item) {
            ret.push(*item);
            s.insert(*item);
        }
    }

    ret
}

fn main() {
    let items = vec![3, 1, 1];
    let ret = find_unique_items(items);
    println!("ret={:?}", ret);
}

#[test]
fn test_find_unique_items() {
    {
        let ret = find_unique_items::<i32>(vec![]);
        assert!(ret.is_empty());
    }
    {
        let items = vec![1, 3];
        let expected = vec![1, 3];
        let ret = find_unique_items(items);
        assert_eq!(ret, expected);
    }
    {
        let items = vec![1, 1, 3];
        let expected = vec![1, 3];
        let ret = find_unique_items(items);
        assert_eq!(ret, expected);
    }
    {
        let items = vec![3, 1, 1];
        let expected = vec![3, 1];
        let ret = find_unique_items(items);
        assert_eq!(ret, expected);
    }
}
