use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

/*
This trait can't fail as per docs, but we're using this as you can't use TryFrom with lifetimes
A query string can be something like a=1&b=2&c&d=&e===&d=7&d=abc
*/
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";

            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing_val: &mut Value| {
                    match existing_val {
                        Value::Single(prev_value) => {
                            /*
                            existing_val is a reference to a mutable Value, so a pointer to and address to a memory inside of the HashMap.
                            By assigning a new value to existing_val without dereferencing we are swapping the pointer/address and not the memory itself.
                            We know that the variants of an enum takes the same space in memory (Value enum in this case), so the memory can be swapped
                            as it is the same size.
                            So there is a need to dereference with the * so the memory is swapped
                            */
                            *existing_val = Value::Multiple(vec![prev_value, val]);
                        }
                        Value::Multiple(vec) => vec.push(val),
                    }
                })
                .or_insert(Value::Single(val));
        }
        QueryString { data }
    }
}
