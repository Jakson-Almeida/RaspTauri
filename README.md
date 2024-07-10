# Guess Game

## Overview
Guess Game is an educational project designed for beginners in Rust, Sycamore, and Tauri. The game generates a random number between 0 and 100, and the user tries to guess it. The game provides feedback if the guess is too high, too low, or correct.

## Technologies
- **Rust**: A system programming language known for performance and safety.
- **Sycamore**: A Rust framework for building reactive web applications.
- **Tauri**: A toolkit for building desktop applications with web technologies and Rust.

## What is WebAssembly?
WebAssembly (Wasm) is a binary instruction format for a stack-based virtual machine. It allows code written in multiple languages to run on the web at near-native speed by taking advantage of common hardware capabilities.

## Game Logic
The game logic is handled by a Tauri command that checks the user's guess against the randomly generated secret number. The code snippet below shows how Tauri decides if the guess is correct:

```rust
#[tauri::command]
fn guess2(name: &str) -> String {
    let mut secret_number = SECRET.lock().unwrap();
    
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
```

Clone the repository:

`git clone https://github.com/Jakson-Almeida/RaspTauri.git`

Navigate to the project directory and install dependencies:

`cd RaspTauri`

`cd src-tauri`

`cargo tauri dev`

Enjoy playing the Guess Game and learning about Rust, Sycamore, and Tauri!
