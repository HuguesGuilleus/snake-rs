use crossterm::event::KeyCode;
use crossterm::event::{poll, read, Event};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rand;
use snake::*;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
                    KeyCode::Esc => Control::Exit,
                    _ => Control::None,
                },
                _ => Control::None,
            },
        }
    };

    let output_view = |s: &str| {
        use crossterm::{
            cursor::MoveTo,
            execute,
            terminal::{Clear, ClearType},
        };
        use std::io::{stdout, Write};

        let mut stdout = stdout();
        execute!(stdout, MoveTo(0, 0), Clear(ClearType::All)).unwrap();
        stdout.write_all(s.as_bytes()).unwrap();
    };

    let size = crossterm::terminal::size()?;
    let size: (usize, usize) = (size.0 as usize, size.1 as usize);

    enable_raw_mode()?;

    Board::new((size.0 as usize, size.1 as usize)).play(
        rand_usize,
        control,
        TextView::new(size, output_view),
    );

    disable_raw_mode()?;
    println!("");
    Ok(())
}

fn rand_usize() -> usize {
    rand::random::<usize>()
}
