// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::io;
use std::cmp::Ordering;
use rand::Rng;
use once_cell::sync::Lazy;
use std::sync::Mutex;

// Variável global estática
static SECRET: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(1000));

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn test(name: &str) -> String {
    format!("Hello, {}! You've been tested from Rust!", name)
}

#[tauri::command]
fn get_random() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[tauri::command]
fn guess(name: &str, secret_number: u32) -> String {
    let my_int: u32 = match name.trim().parse() {
        Ok(num) => num,
        Err(_) => return "Invalid input! Please enter a number.".to_string(),
    };

    match my_int.cmp(&secret_number) {
        Ordering::Less => format!("{} is too small!", my_int),
        Ordering::Greater => format!("{} is too big!", my_int),
        Ordering::Equal => format!("{} is correct! You win!", my_int),
    }
}

#[tauri::command]
fn guess2(name: &str) -> String {
    let mut secret_number = SECRET.lock().unwrap();
    
    // Defina o número secreto fixo
    if *secret_number == 1000 {
        *secret_number = get_random();
    }

    let my_int: u32 = match name.trim().parse() {
        Ok(num) => num,
        Err(_) => return "Invalid input! Please enter a number.".to_string(),
    };

    match my_int.cmp(&*secret_number) {
        Ordering::Less => format!("{} is too small!", my_int),
        Ordering::Greater => format!("{} is too big!", my_int),
        Ordering::Equal => {
            *secret_number = 1000;
            format!("{} is correct! You win!", my_int)
        },
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, test, guess, guess2, get_random])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
