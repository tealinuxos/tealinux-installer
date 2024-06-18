// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::{File, OpenOptions}, io::{Read, Write}};

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct User{
  username: String,
  password: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Root{
  password: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Hostname{
  name: String
}

fn read_json(file_path: &str) -> serde_json::Value {
  let file = match File::open(file_path) {
      Ok(mut file) => {
          let mut contents = String::new();
          file.read_to_string(&mut contents).unwrap();
          serde_json::from_str(&contents).unwrap_or_else(|_| json!({}))
      },
      Err(_) => json!({}),
  };

  file
}

fn write_json(file_path: &str, data: &serde_json::Value) {
  let json_str = serde_json::to_string_pretty(data).unwrap();
  let mut file = OpenOptions::new()
      .write(true)
      .truncate(true)
      .create(true)
      .open(file_path)
      .unwrap();
  file.write_all(json_str.as_bytes()).unwrap();
}

#[tauri::command]
fn create_user(username: String, password: String){
  let path_json = "../src/data/Meta.json";
  let mut json = read_json(path_json);

  let user = User{
    username,
    password
  };

  json["User"] = json!({
    "username": user.username,
    "password": user.password
  });

  write_json(path_json, &json);
}

#[tauri::command]
fn create_root(password: String){
  let path_json = "../src/data/Meta.json";
  let mut json = read_json(path_json);

  let root = Root{
    password
  };

  json["Root"] = json!({
    "password": root.password
  });

  write_json(path_json, &json);
}

#[tauri::command]
fn create_hostname(name: String){
  let path_json = "../src/data/Meta.json";
  let mut json = read_json(path_json);

  let hostname = Hostname{
    name
  };

  json["Hostname"] = json!({
    "name": hostname.name
  });

  write_json(path_json, &json);
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      create_user, 
      create_root, 
      create_hostname
      ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}