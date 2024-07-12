use std::io::{stdout, Write};
use crossterm::{
    event::{KeyCode, Event, read},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};

fn main() {
    clear();
    show_title_screen();
    playground();
    // TODO:
    // code the second page of tippy-toes typing.
}

fn show_title_screen() {
    write(Color::Blue, "Tippy-Toes Typing! (press enter)\n");
    wait_for_key_press();
    clear();
    write(Color::Yellow, "Press ENTER to start game...\n");
    wait_for_key_press();
    clear();
}

fn playground() {
    // This is your playground
    write(Color::Blue, "Hello random person who wants to type properly (press enter)\n");
    wait_for_key_press();
    clear();
    write(Color::Yellow, "\n");
}
fn clear() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
}

fn write(color: Color, text: &str) {
    execute!(
        stdout(),
        SetForegroundColor(color),
        Print(text),
        ResetColor,
    ).unwrap();
}

fn wait_for_key_press() {
    loop {
        if let Ok(Event::Key(key_event)) = read() {
            match key_event.code {
                KeyCode::Enter => {
                    println!("Enter key pressed");
                    break;
                }
                _ => {
                    // Handle other key presses if needed
                    println!("Other key pressed");
                }
            }
        }
    } 
}
