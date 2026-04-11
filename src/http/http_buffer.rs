use memchr::memmem;

pub struct HttpBuffer {
    value: Vec<u8>,
    headers_end: Option<usize>,
}

impl HttpBuffer {
    pub const MAX_HEADERS_BYTES: usize = 8192;
    pub const HEADERS_END_PATTERN: &[u8; 4] = b"\r\n\r\n";

    pub fn new() -> Self {
        Self {
            value: Vec::new(),
            headers_end: None,
        }
    }

    pub fn add(&mut self, slice: &[u8]) {
        self.value.extend_from_slice(slice);

        if self.headers_end.is_none() {
            self.headers_end = self.get_headers_end();
        }
    }

    pub fn split(&self) -> Option<(&[u8], &[u8], &[u8])> {
        let end: usize = self.headers_end?;

        let pre_header: &[u8] = &self.value[..end];
        let body: &[u8] = &self.value[end..];

        let request_line: &[u8] = Self::get_request_line(pre_header)?;
        let headers: &[u8] = Self::get_headers(pre_header)?;

        Some((request_line, headers, body))
    }

    fn get_request_line(pre_header: &[u8]) -> Option<&[u8]> {
        memmem::find(pre_header, b"\r\n").map(|position| &pre_header[..position])
    }

    fn get_headers(pre_header: &[u8]) -> Option<&[u8]> {
        let start: usize = memmem::find(pre_header, b"\r\n")? + 2;
        let end: usize = pre_header.len();

        Some(&pre_header[start..end - 4])
    }

    fn get_headers_end(&self) -> Option<usize> {
        let headers_end: Option<usize> = self
            .value
            .windows(4)
            .position(|window| window == HttpBuffer::HEADERS_END_PATTERN);

        if let Some(position) = headers_end {
            return Some(position + 4);
        }

        None
    }

    pub fn exceeded_max_bytes(&self) -> bool {
        self.value.len() > Self::MAX_HEADERS_BYTES
    }
}
