use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Headers<'a> {
    data: HashMap<&'a str, &'a str>
}

impl<'a> Headers<'a> {
    pub fn new(data: HashMap<&'a str, &'a str>) -> Self {
        Self { data }
    }
}

impl<'a> Display for Headers<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut headers_as_string = String::new();

        for (key, val) in self.data.iter() {
            headers_as_string.push_str(&format!("{}: {}\r\n", key, val))
        }

        write!(f, "{}", headers_as_string)
    }
}

impl<'a> From<&'a str> for Headers<'a> {
    fn from(s: &'a str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split("\r\n") {
            let mut key = sub_str;
            let mut val = "";

            if sub_str.contains("\r\n") {
                break
            }

            if let Some(i) = sub_str.find(':') {
                key = &sub_str[..i].trim();
                val = &sub_str[i + 1..].trim();
            }

            data.entry(key)
                .or_insert(val);
        }

        Headers{ data }
    }
}