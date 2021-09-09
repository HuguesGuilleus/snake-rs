// cargo build --target wasm32-unknown-unknown --release

use snake::*;

extern "C" {
    fn random() -> u32;
    fn screen(ptr: u32, len: u32);
}

struct Playground<'a> {
    board: Board,
    view: TextView<fn(&str)>,
    party: Option<Party<'a, TextView<fn(&str)>>>,
}

static mut PLAYGROUND: Option<Playground<'_>> = None;

#[no_mangle]
pub extern "C" fn init(width: u32, height: u32) {
    let size = (width as usize, height as usize);

    fn rand() -> usize {
        unsafe { random() as usize }
    }

    unsafe {
        PLAYGROUND = Some(Playground {
            board: Board::new(size),
            view: TextView::new(size, output),
            party: None,
        });
        if let Some(ref mut p) = PLAYGROUND {
            p.party = Some(p.board.party(rand, &mut p.view));
        }
    }
}

#[no_mangle]
pub extern "C" fn step(control: u32) -> u32 {
    let control = match control {
        1 => Control::Left,
        2 => Control::Right,
        3 => Control::Up,
        4 => Control::Down,
        _ => Control::None,
    };

    let b = unsafe {
        match PLAYGROUND {
            Some(Playground {
                party: Some(ref mut p),
                ..
            }) => p.step(control),
            _ => {
                output(
                "The playground is non initialized, please call `env/init(with, height)` before.",
            );
                false
            }
        }
    };

    match b {
        false => 0,
        true => 1,
    }
}

fn output(s: &str) {
    unsafe {
        screen(s.as_ptr() as u32, s.len() as u32);
    }
}
