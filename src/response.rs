use crate::*;

#[derive(Debug, Clone, Copy)]
pub enum Status {
    OK,
    NotFound,
}

impl core::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::OK => write!(f, "200 OK"),
            Status::NotFound => write!(f, "404 NOT_FOUND"),
        }
    }
}

#[derive(Debug)]
pub struct Response {
    pub protocol: String,
    pub status: Status,
    pub headers: Headers,
    pub content: String,
}

impl Response {
    pub fn new(protocol: &str, status: &Status, content: &str) -> Self {
        Self {
            protocol: protocol.to_string(),
            status: *status,
            headers: Headers::new(),
            content: content.to_string(),
        }
    }

    pub fn raw(&self) -> String {
        let mut headers = self
            .headers
            .map
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\r\n");
        if !headers.is_empty() {
            headers = format!("\r\n{}", headers);
        }

        format!(
            "{} {}{}\r\n\r\n{}",
            self.protocol, self.status, headers, self.content
        )
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }
}
