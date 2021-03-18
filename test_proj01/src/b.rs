use crate::a::Sample;

pub fn func(name: String) -> String {
    let s = Sample::new(name);
    s.hello()
}
