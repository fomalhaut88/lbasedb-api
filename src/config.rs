use std::env;


pub struct Config {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub data_path: String,
    pub payload_limit: usize,
}


impl Config {
    pub fn from_env() -> Self {
        Self {
            host: env::var("HOST").unwrap_or("localhost".to_string()),
            port: env::var("PORT").unwrap_or("8080".to_string())
                                  .parse().expect("PORT must be integer"),
            workers: env::var("WORKERS").unwrap_or("1".to_string())
                                        .parse()
                                        .expect("WORKERS must be integer"),
            data_path: env::var("DATA_PATH").unwrap_or("./tmp/db".to_string()),
            payload_limit: env::var("PAYLOAD_LIMIT")
                            .unwrap_or("1073741824".to_string())
                            .parse().expect("PAYLOAD_LIMIT must be integer"),
        }
    }
}
