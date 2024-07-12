# Typing Game

## How to

```sh
cargo run # Run. Use it inside `typing` folder.
```

## VSCode Hotkeys

- `Ctrl + J`: Open/ Close command console.
- `Ctrl + B`: Open/ Close sidebar
- `Ctrl + c`: Stop running code

## Naming rule

"I love you".

- Snake case: `i_love_you`
- Camel case: `iLoveYou`
- Capital camel case: `ILoveYou`

## Tongtong's note:

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
    show_title_screen();
    // TODO:
    // code the scond page of tippy-toes typing.
}

fn playground() {
    // This is your playground
    write(Color::Blue, "Hello random person who wants to type properly (enter to move on)\n");
    wait_for_key_press();
    clear();
    write(Color::Yellow, "\n");
}
//}

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

st
Copy code
use std::io;

fn main() {
    println!("Enter something:");

    // Create a mutable string to store the input
    let mut input = String::new();

    // Read input from the user
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    // Print the input back to the user
    println!("You entered: {}", input);
}