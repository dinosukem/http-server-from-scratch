use crate::*;

#[derive(Debug)]
pub struct Response {
    pub protocol: String,
    pub status: String,
    pub headers: Headers,
    pub content: String,
}

impl Response {
    pub fn new(protocol: &str, status: &str, content: &str) -> Self {
        Self {
            protocol: protocol.to_string(),
            status: status.to_string(),
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
