#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Data {
    name: String,
    age: u32,
}

impl Data {
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
}

fn main() {
    let mut datas = vec![
        Data::new("taro".to_string(), 39),
        Data::new("ichiro".to_string(), 39),
        Data::new("jiro".to_string(), 80),
        Data::new("saburo".to_string(), 56),
    ];

    datas.sort_unstable();

    println!("## sorted={:?}", datas);

    datas.sort_unstable_by(|a, b| match a.age.cmp(&b.age) {
        std::cmp::Ordering::Equal => a.name.cmp(&b.name),
        v => v,
    });

    println!("## sort by custom func={:?}", datas);
}
