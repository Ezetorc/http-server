#[derive(Debug)]
pub enum Body {
    Text(String),
    Binary(Vec<u8>),
}

impl Body {
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            Self::Text(string) => string.as_bytes().to_vec(),
            Self::Binary(bytes) => bytes.clone(),
        }
    }
}

impl From<String> for Body {
    fn from(value: String) -> Self {
        Body::Text(value)
    }
}

impl From<&str> for Body {
    fn from(value: &str) -> Self {
        Body::Text(value.to_string())
    }
}

impl From<Vec<u8>> for Body {
    fn from(value: Vec<u8>) -> Self {
        Body::Binary(value)
    }
}
