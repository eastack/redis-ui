use std::{
    fs::File,
    io::{BufReader, BufWriter},
};
use ts_rs::TS;
use uuid::Uuid;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DatabaseDto {
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

#[derive(Debug, Clone)]
pub struct Databases {
    filepath: String,
    pub dbs: Vec<Database>,
}

impl Databases {
    pub fn new(filename: &str) -> Self {
        let file = File::open(filename).expect("open file failed");
        let file = BufReader::new(file);
        Databases {
            filepath: filename.to_string(),
            dbs: serde_json::from_reader::<_, Vec<Database>>(file).unwrap_or(vec![])
        }
    }

    pub fn add(&mut self, cmd: DatabaseDto) {
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

        self.dbs.push(db);

        self.save();
    }

    pub fn remove(&mut self, id: &str) {
        println!("Remove db");

        self.dbs.retain(|db| db.id != id);

        self.save();
    }

    pub fn update(&mut self, id: &str, cmd: DatabaseDto) {
        println!("Update db");
        if let Some(db) = self.dbs.iter_mut().find(|db| db.id == id) {
            db.name = cmd.name;
            db.comment = cmd.comment;
            db.color = cmd.color;
            db.host = cmd.host;
            db.port = cmd.port;
            db.auth = cmd.auth;
            db.db = cmd.db;
        }

        self.save();
    }

    pub fn list(self, q: Option<&str>) -> Option<Vec<Database>> {
        let res = match q {
            Some(q) => self.dbs.into_iter().filter(|db: &Database| db.name.contains(q)).collect(),
            None => self.dbs.into_iter().collect(),
        };
        Some(res)
    }

    pub fn save(&self) {
        let config = File::create(&self.filepath).expect("config file open failed");
        let config = BufWriter::new(config);
        serde_json::to_writer(config, &self.dbs).expect("config write failed");
    }

    pub fn get(self, id: &str) -> Option<Database> {
        self.dbs.into_iter().find(|db| db.id == id)
    }
}
