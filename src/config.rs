pub struct Config {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}


impl Config {
    pub fn new() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8080,
            workers: 16,
        }
    }
}
