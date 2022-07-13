#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod database;
use redis::Commands;
use redis::Connection;
use tauri::LogicalSize;
use tauri::Size;

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
    let dbs = Databases::new("/tmp/redis-ui-config.json");

    let context = tauri::generate_context!();
    let exit = CustomMenuItem::new("exit".to_string(), "Exit");
    let submenu = Submenu::new("File", Menu::new().add_item(exit));
    let menu = Menu::new().add_submenu(submenu);

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
            "close_window" => {
                event.window().close().unwrap();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            ping, keys, add_db, remove_db, update_db, list_db, get_db, open_docs
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

#[tauri::command]
async fn open_docs(handle: tauri::AppHandle) {
    let exit = CustomMenuItem::new("close_window".to_string(), "Close Window");
    let submenu = Submenu::new("File", Menu::new().add_item(exit));
    let menu = Menu::new().add_submenu(submenu);

    let window = tauri::WindowBuilder::new(
        &handle,
        "New", /* the unique window label */
        tauri::WindowUrl::External("http://localhost:3000/db/create".parse().unwrap()),
    )
    .center()
    .menu(menu)
    .build()
    .unwrap();

    window
        .set_min_size(Some(Size::Logical(LogicalSize {
            width: 61_f64,
            height: 100_f64,
        })))
        .unwrap();

    window
        .set_size(Size::Logical(LogicalSize {
            width: 618_f64,
            height: 1000_f64,
        }))
        .unwrap();
    //window.set_resizable(false).unwrap();
}
