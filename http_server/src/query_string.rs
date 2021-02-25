use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'a> {
    data: HashMap<&'a str, Value<'a>>,
}

#[derive(Debug)]
pub enum Value<'a> {
    Single(&'a str),
    Multiple(Vec<&'a str>),
}

impl<'a> QueryString<'a> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'a> From<&'a str> for QueryString<'a> {
    fn from(s: &'a str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            data.entry(key)
                .and_modify(|v: &mut Value| match v {
                    Value::Single(old_value) => *v = Value::Multiple(vec![old_value, val]),
                    Value::Multiple(old_values) => old_values.push(val),
                })
                .or_insert(Value::Single(val));
        }

        Self { data }
    }
}
