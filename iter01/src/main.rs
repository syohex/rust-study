fn test01() {
    let mut vec_iter = (40..46).collect::<Vec<_>>().into_iter();

    println!("first three:");
    for (i, n) in vec_iter.by_ref().take(3).enumerate() {
        println!("{}: {}", i, n);
    }

    println!("rest: {}", vec_iter.len());
    for (i, n) in vec_iter.enumerate() {
        println!("{}: {}", i, n);
    }
}

fn main() {
    let v = [1, 2, 3];
    v.iter().for_each(|x| println!("{}", x));
    let vv: Vec<i32> = v.iter().map(|x| x * x).collect();
    println!("{:?}", vv);

    test01();
}
