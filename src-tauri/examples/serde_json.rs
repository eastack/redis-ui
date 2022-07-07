use std::{fs::File, io::{Write, BufWriter}};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    name: String,
    host: String,
    port: u8,
}
fn main() {
    let file = File::create("/home/radix10/tmp.json").unwrap();

    let config = Config {
        name: "Test".to_string(),
        host: "local".to_string(),
        port: 0,
    };

    let str = serde_json::to_string(&config).unwrap();
    println!("{str}");
    let mut file = BufWriter::new(file);
    serde_json::to_writer(&mut file, &config).unwrap();
    file.flush().unwrap();
}
