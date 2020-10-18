use std::collections::{HashMap};
#[derive(Debug)]
pub struct QueryString<'buf>{
    data: HashMap<&'buf str, Value<'buf>>,
}
#[derive(Debug)]
pub enum Value<'buf>{
    Single(&'buf str),
    Multipe(Vec<&'buf str>), //Vector is a tab allocated in heap, so i can grow after is allocation.
}

impl<'buf> QueryString<'buf>{
    pub fn get(&self, key: &str) -> Option<&Value>{
        return self.data.get(key);
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf>{
    fn from(s: &'buf str) -> Self{
        let mut data = HashMap::new();

        for s_string in s.split("&"){
            let mut key = s_string;
            let mut valu = "";
            let mut valu_exit = s_string.find("=");
            if valu_exit.is_some() {
                let i = valu_exit.unwrap();
                key = &s_string[..i];
                valu = &s_string[i + 1 ..];
            }
            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                        *existing = Value::Multipe(vec![prev_val, valu]);
                    }
                    Value::Multipe(vec) => vec.push(valu),
                })
                .or_insert(Value::Single(valu));
        }
        return QueryString{data};
    }
}