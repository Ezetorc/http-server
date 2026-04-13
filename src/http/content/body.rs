#[derive(Debug)]
pub enum HttpBody {
    Text(String),
    Binary(Vec<u8>),
}

impl HttpBody {
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            Self::Text(string) => string.as_bytes().to_vec(),
            Self::Binary(bytes) => bytes.clone(),
        }
    }

    pub fn as_string(&self) -> Option<String> {
        match self {
            Self::Text(string) => Some(string.clone()),
            Self::Binary(bytes) => String::from_utf8(bytes.clone()).ok(),
        }
    }
}

impl From<String> for HttpBody {
    fn from(value: String) -> Self {
        HttpBody::Text(value)
    }
}

impl From<&str> for HttpBody {
    fn from(value: &str) -> Self {
        HttpBody::Text(value.to_string())
    }
}

impl From<Vec<u8>> for HttpBody {
    fn from(value: Vec<u8>) -> Self {
        HttpBody::Binary(value)
    }
}
