#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod config;
use redis::Commands;

use std::sync::{Arc, Mutex};

use config::Config;
use tauri::{CustomMenuItem, Menu, Submenu};

use commands::add_db;
use commands::list_db;
use commands::remove_db;

#[derive(Clone)]
pub struct Core {
    pub config: Arc<Mutex<Config>>,
}

fn main() {
    let context = tauri::generate_context!();

    let exit = CustomMenuItem::new("exit".to_string(), "Exit");
    let submenu = Submenu::new("File", Menu::new().add_item(exit));
    let menu = Menu::new().add_submenu(submenu);

    let config = Config::new("/tmp/redis-ui-config.json");
    println!("current config is: {config:?}");

    tauri::Builder::default()
        .menu(menu)
        .manage(Core {
            config: Arc::new(Mutex::new(config)),
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
            ping, keys, list_db, add_db, remove_db
        ])
        .run(context)
        .expect("error while running tauri application");
}

#[tauri::command]
fn ping() -> String {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();
    let cmd = redis::cmd("PING");
    cmd.query(&mut con).unwrap()
}

#[tauri::command]
fn keys(key_pattern: &str) -> Vec<String> {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();
    let keys: Vec<String> = con.keys(key_pattern.to_string()).unwrap();
    println!("current pattern: {key_pattern} current keys: {keys:?}");
    keys
}
