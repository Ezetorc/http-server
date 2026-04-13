use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpHeaders {
    headers: HashMap<String, String>,
}

impl HttpHeaders {
    pub fn get(&self, header_name: &str) -> Option<&String> {
        self.headers.get(header_name)
    }

    pub fn has(&self, header_name: &str) -> bool {
        self.headers.contains_key(header_name)
    }
}

impl From<String> for HttpHeaders {
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
