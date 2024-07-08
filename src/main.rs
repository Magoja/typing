use std::io::{stdout};
use crossterm::{
    event::{KeyCode, Event, read},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};

fn main() {
    // Clear the screen
    execute!(stdout(), Clear(ClearType::All)).unwrap();
    
    // Change text color to Blue and print a message
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        Print("Tippy-Toes Typing! (press enter)\n"),
        ResetColor,
    ).unwrap();

    // Wait for the user to press any key
    wait_for_key_press();

    // Clear the screen again
    execute!(stdout(), Clear(ClearType::All)).unwrap();

    // Change text color to Yellow and print another message
    execute!(
        stdout(),
        SetForegroundColor(Color::Yellow),
        Print("Press ENTER to start typing..."),
        ResetColor,
    ).unwrap();

    // Further code execution can continue here
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
