use std::{
    collections::BTreeMap,
    io::{
        BufRead,
        BufReader,
    },
};

#[derive(Debug, Clone)]
pub struct Path {
    pub elements: Vec<String>,
}

impl Path {
    pub fn new(path: &str) -> Self {
        let elements = path
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        Self { elements }
    }

    pub fn segments(&self) -> Vec<&str> {
        self.elements.iter().map(String::as_str).collect()
    }
}

#[derive(Debug, Default)]
pub struct Headers {
    pub map: BTreeMap<String, String>,
}

impl Headers {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn get(&self, key: &str) -> String {
        self.map.get(key).unwrap_or(&"".to_string()).clone()
    }
}

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: Path,
    pub protocol: String,
    pub headers: Headers,
}

impl Request {
    pub fn from_stream(mut stream: std::net::TcpStream) -> Self {
        let buf = BufReader::new(&mut stream);

        let request_lines: Vec<String> = buf
            .lines()
            .map(|line| line.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let metadata = request_lines[0].split_whitespace().collect::<Vec<&str>>();

        Self {
            method: metadata[0].to_string(),
            path: Path::new(metadata[1]),
            protocol: metadata[2].to_string(),
            headers: {
                let mut headers = Headers::new();
                for line in request_lines[1..].iter() {
                    let parts: Vec<&str> = line.splitn(2, ": ").collect();
                    if parts.len() == 2 {
                        headers.insert(parts[0].to_string(), parts[1].to_string());
                    }
                }
                headers
            },
        }
    }
}
