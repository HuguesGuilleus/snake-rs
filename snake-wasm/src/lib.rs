// cargo build --target wasm32-unknown-unknown --release

use snake::*;

/// Message used if an call of [`step`] before an [`init`] call.
static NONE_PARTY: &str =
    "The playground is non initialized, please call `env/init(with, height)` before.";

static mut BOARDANDTEXTVIEW: Option<(Board, TextView<fn(&str)>)> = None;
static mut PARTY: Option<Party<'_, TextView<fn(&str)>>> = None;

#[link(wasm_import_module = "js")]
extern "C" {
    /// Get a random inbteger.
    fn random() -> u32;
    /// Send a string that will be printed to the sceen. ptr an len is the string pointer an length.
    fn screen(ptr: u32, len: u32);
}

/// Call [`screen`] external function with ptr and len of the string s.
fn output(s: &str) {
    unsafe {
        screen(s.as_ptr() as u32, s.len() as u32);
    }
}

/// Init the playground, it's necesary to call it before [`step`] function call.
#[no_mangle]
pub extern "C" fn init(width: u32, height: u32) {
    let size = (width as usize, height as usize);

    fn rand() -> usize {
        unsafe { random() as usize }
    }

    unsafe {
        BOARDANDTEXTVIEW = Some((Board::new(size), TextView::new(size, output)));
        let (ref mut board, ref mut view) = BOARDANDTEXTVIEW.as_mut().unwrap();
        PARTY = Some(board.party(rand, view));
    }
}

/// Compute one party step with the user control command.
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
        match PARTY {
            Some(ref mut p) => p.step(control),
            None => {
                output(NONE_PARTY);
                false
            }
        }
    };

    match b {
        false => 0,
        true => 1,
    }
}
