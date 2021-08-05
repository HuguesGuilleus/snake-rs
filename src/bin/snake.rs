use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use crossterm::event::{poll, read, Event};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use snake_rs::*;
use std::pin::Pin;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let size = crossterm::terminal::size()?;
    println!("size: {:?}", size);

    enable_raw_mode()?;

    let control = || -> Control {
        match poll(Duration::from_millis(100)) {
            Err(_) => Control::Exit,
            Ok(false) => Control::None,
            Ok(true) => match read() {
                Err(_) => Control::Exit,
                Ok(Event::Key(k)) => match k.code {
                    KeyCode::Left => Control::Left,
                    KeyCode::Right => Control::Right,
                    KeyCode::Up => Control::Up,
                    KeyCode::Down => Control::Down,
                    KeyCode::Down => Control::Down,
                    KeyCode::Esc => Control::Exit,
                    _ => Control::None,
                },
                _ => Control::None,
            },
        }
    };

    Board::new((size.0 as usize, size.1 as usize)).play(control);

    disable_raw_mode()?;
    Ok(())
}
