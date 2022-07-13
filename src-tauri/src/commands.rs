use crate::{
    database::{Database, DatabaseDto},
    Core,
};
use tauri::State;

#[tauri::command]
pub fn list_db<'a>(core: State<'_, Core>, q: Option<&str>) -> Option<Vec<Database>> {
    println!("List db {}", q);

    let db = core.db.lock().unwrap().clone();
    db.list(q)
}

#[tauri::command]
pub fn add_db(core: State<'_, Core>, db: DatabaseDto) {
    core.db.lock().unwrap().add(db);
}

#[tauri::command]
pub fn remove_db(core: State<'_, Core>, id: &str) {
    core.db.lock().unwrap().remove(id);
}

#[tauri::command]
pub fn update_db(core: State<'_, Core>, id: &str, db: DatabaseDto) {
    println!("update db: {:?}", db);
    core.db.lock().unwrap().update(id, db);
}

#[tauri::command]
pub fn get_db(core: State<'_, Core>, id: &str) -> Option<Database> {
    let db = core.db.lock().unwrap().clone();
    db.get(id)
}

#[tauri::command]
pub fn ping(core: State<'_, Core>, id: &str) -> String {
    let db = core.db.lock().unwrap().clone();
    let db = db.get(id);

    let conn = db
        .map(|db| format!("redis://{}/", db.host))
        .map(|param| redis::Client::open(param).unwrap())
        .unwrap();

    let mut con = conn.get_connection().unwrap();
    let cmd = redis::cmd("PING");
    cmd.query(&mut con).unwrap()
}
