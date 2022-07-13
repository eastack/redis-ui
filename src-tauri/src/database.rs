use pinyin::{Pinyin, ToPinyin};
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
    pub host: String,
    pub port: u16,
    pub alias: String,
    pub db: u16,
    pub username: String,
    pub password: String,
    pub tls: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Database {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub alias: String,
    pub db: u16,
    pub username: String,
    pub password: String,
    pub tls: bool,
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
            dbs: serde_json::from_reader::<_, Vec<Database>>(file).unwrap_or(vec![]),
        }
    }

    pub fn add(&mut self, cmd: DatabaseDto) {
        println!("Add db");

        let db = Database {
            id: Uuid::new_v4().to_string(),
            host: cmd.host,
            port: cmd.port,
            alias: cmd.alias,
            db: cmd.db,
            username: cmd.username,
            password: cmd.password,
            tls: cmd.tls,
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
            db.host = cmd.host;
            db.port = cmd.port;
            db.alias = cmd.alias;
            db.db = cmd.db;
        }

        self.save();
    }

    pub fn list(self, q: Option<&str>) -> Option<Vec<Database>> {
        let res = match q {
            Some(q) => self
                .dbs
                .into_iter()
                .filter(|db: &Database| {
                    db.alias
                        .as_str()
                        .to_pinyin()
                        .flatten()
                        .map(Pinyin::first_letter)
                        .collect::<Vec<&str>>()
                        .join("")
                        .contains(q)
                        || db.host.contains(q)
                        || db.port.to_string().contains(q)
                })
                .collect(),
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
