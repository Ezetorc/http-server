use std::{
    collections::{HashMap, hash_map::Iter},
    str::FromStr,
};

use crate::http::response::content_type::ContentType;

#[derive(Debug)]
pub struct Headers {
    headers: HashMap<String, String>,
}

impl Headers {
    pub const CONTENT_LENGTH: &str = "Content-Length";
    pub const CONTENT_TYPE: &str = "Content-Type";

    pub fn new() -> Self {
        Self {
            headers: HashMap::new(),
        }
    }

    pub fn get(&self, header_name: &str) -> Option<&String> {
        self.headers.get(header_name.to_lowercase().as_str())
    }

    pub fn get_as<Type>(&self, header_name: &str) -> Option<Type>
    where
        Type: FromStr,
    {
        self.get(header_name)
            .and_then(|value| value.parse::<Type>().ok())
    }

    pub fn set_content_type(&mut self, content_type: ContentType) {
        self.set(Self::CONTENT_TYPE, content_type.as_str());
    }

    pub fn has(&self, header_name: &str) -> bool {
        self.headers.contains_key(header_name)
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn iter(&self) -> Iter<'_, String, String> {
        self.headers.iter()
    }
}

impl From<String> for Headers {
    fn from(string: String) -> Self {
        let mut headers: HashMap<String, String> = HashMap::new();

        let lines = string.split("\r\n");

        for line in lines {
            if let Some((key, value)) = line.split_once(':') {
                headers.insert(key.to_string(), value.to_string());
            }
        }

        Self { headers }
    }
}
