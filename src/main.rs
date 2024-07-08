use std::io::{stdout, Write};
use crossterm::{execute, terminal::{Clear, ClearType}};

fn main() {
    clearScreen();
    println!("Hello Tongtong!");
}

fn clearScreen() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
}
