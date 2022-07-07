use std::{
    fs::File,
    io::{BufReader, BufWriter},
};
use ts_rs::TS;
use uuid::Uuid;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Config {
    filename: String,
    pub databases: Option<Vec<Database>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct NewDatabaseCmd {
    pub name: String,
    pub comment: String,
    pub color: String,
    pub host: String,
    pub port: u16,
    pub auth: RedisAuth,
    pub db: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Database {
    pub id: String,
    pub name: String,
    pub comment: String,
    pub color: String,
    pub host: String,
    pub port: u16,
    pub auth: RedisAuth,
    pub db: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum RedisAuth {
    None,
    Password { username: String, password: String },
}

impl Config {
    pub fn new(filename: &str) -> Self {
        let config = File::open(filename).expect("open file failed");
        let config = BufReader::new(config);
        Config {
            filename: filename.to_string(),
            databases: serde_json::from_reader::<_, Vec<Database>>(config).ok(),
        }
    }

    pub fn add_db(&mut self, cmd: NewDatabaseCmd) {
        println!("Add db");

        let db = Database {
            id: Uuid::new_v4().to_string(),
            name: cmd.name,
            comment: cmd.comment,
            color: cmd.color,
            host: cmd.host,
            port: cmd.port,
            auth: cmd.auth,
            db: cmd.db,
        };

        self.databases.get_or_insert_with(|| vec![]).push(db);

        let config = File::create(&self.filename).expect("config file open failed");
        let config = BufWriter::new(config);
        serde_json::to_writer(config, &self.databases).expect("config write failed");
    }

    pub fn remove_db(&mut self, id: &str) {
        println!("Remove db");

        if let Some(dbs) = &mut self.databases {
            dbs.retain(|db| db.id != id)
        }

        let config = File::create(&self.filename).expect("config file open failed");
        let config = BufWriter::new(config);
        serde_json::to_writer(config, &self.databases).expect("config write failed");
    }
}
