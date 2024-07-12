use std::io::{stdout};
use crossterm::{
    event::{KeyCode, Event, read},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};

fn main() {
    clear();
    playground();
    // show_title_screen();
    // TODO:
    // code the scond page of tippy-toes typing.
}

fn playground() {
    // This is your playground
    write(Color::Blue, "Wahahahah\n");
}

fn show_title_screen() {
    write(Color::Blue, "Tippy-Toes Typing! (press enter)\n");
    wait_for_key_press();
    clear();
    write(Color::Yellow, "Press ENTER to start typing...\n");
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
    let event = read().unwrap();
    // Handle the event
    match event {
        Event::Key(key_event) => {
            match key_event.code {
                KeyCode::Char(c) => println!("Character pressed: {}", c),
                KeyCode::Enter => println!("Enter key pressed"),
                KeyCode::Esc => println!("Escape key pressed"),
                _ => println!("Other key pressed"),
            }
        }
        _ => println!("Other event"),
    }
}
