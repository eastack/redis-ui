#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod database;
use redis::Commands;
use redis::Connection;

use std::sync::{Arc, Mutex};

use database::Databases;
use tauri::{CustomMenuItem, Menu, Submenu};

use commands::*;

#[derive(Clone)]
pub struct Core {
    pub db: Arc<Mutex<Databases>>,
    pub conn: Arc<Mutex<Option<Connection>>>,
}

fn main() {
    let context = tauri::generate_context!();

    let exit = CustomMenuItem::new("exit".to_string(), "Exit");
    let submenu = Submenu::new("File", Menu::new().add_item(exit));
    let menu = Menu::new().add_submenu(submenu);

    let dbs = Databases::new("/tmp/redis-ui-config.json");

    tauri::Builder::default()
        .menu(menu)
        .manage(Core {
            db: Arc::new(Mutex::new(dbs)),
            conn: Default::default(),
        })
        .on_menu_event(move |event| match event.menu_item_id() {
            "exit" => {
                std::process::exit(0);
            }
            "exit_new_db" => {
                event.window().hide().unwrap();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            ping, keys, add_db, remove_db, update_db, list_db, get_db
        ])
        .run(context)
        .expect("error while running tauri application");
}

#[tauri::command]
fn keys(key_pattern: &str) -> Vec<String> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();
    let keys: Vec<String> = con.keys(key_pattern.to_string()).unwrap();
    println!("current pattern: {key_pattern} current keys: {keys:?}");
    keys
}
