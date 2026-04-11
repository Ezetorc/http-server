#[derive(Debug)]
pub struct HttpBody {
    body: Vec<u8>,
}

impl HttpBody {
    pub fn new(body: &[u8]) -> Self {
        Self {
            body: Vec::from(body),
        }
    }
}
