fn main() {
    let v = [1, 2, 3];
    v.iter().for_each(|x| println!("{}", x));
    let vv: Vec<i32> = v.iter().map(|x| x * x).collect();
    println!("{:?}", vv);
}
