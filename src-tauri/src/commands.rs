use crate::{
    config::{Database, NewDatabaseCmd},
    Core,
};
use tauri::State;

#[tauri::command]
pub fn list_db(core: State<'_, Core>, _q: Option<String>) -> Option<Vec<Database>> {
    println!("List db");
    let config = core.config.lock().unwrap();
    let dbs = config.clone().databases;
    println!("current dbs: {:?}", dbs);
    dbs
}

#[tauri::command]
pub fn add_db(core: State<'_, Core>, db: NewDatabaseCmd) {
    let mut config = core.config.lock().unwrap();
    config.add_db(db);
}

#[tauri::command]
pub fn remove_db(core: State<'_, Core>, id:&str) {
    let mut config = core.config.lock().unwrap();
    config.remove_db(id);
}
