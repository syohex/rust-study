fn case_insensitive_sort<'a>(users: &mut Vec<&'a str>) -> Vec<&'a str> {
    users.sort_by_cached_key(|a| a.to_lowercase());
    users.to_vec()
}

fn main() {
    let mut users = vec!["Todd", "amy"];
    let ret = case_insensitive_sort(&mut users);
    println!("ret={:?}", ret);
}

#[test]
fn test_case_insensitive_sort() {
    {
        let mut users = vec!["Todd", "amy"];
        let expected = vec!["amy", "Todd"];
        let ret = case_insensitive_sort(&mut users);
        assert_eq!(ret, expected);
    }
    {
        let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];
        let expected = vec!["alison", "Amy", "Jennifer", "mike99", "Todd"];
        let ret = case_insensitive_sort(&mut users);
        assert_eq!(ret, expected);
    }
}
