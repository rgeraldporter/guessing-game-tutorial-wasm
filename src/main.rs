extern crate rand;

use rand::Rng;
use std::os::raw::c_char;
use std::ffi::CString;
use std::cmp::Ordering;

pub struct Game {
    value: i32,
}

impl Game {
    fn new() -> Game {
        Game { value: rand::thread_rng().gen_range(1, 101) }
    }

    fn get_value(&self) -> i32 {
        self.value
    }
}

#[no_mangle]
pub fn create_game() -> *mut Game {
    Box::into_raw(Box::new(Game::new()))
}

#[no_mangle]
pub unsafe fn guess_number(game: *const Game, guess: i32) -> *mut c_char {
    let game = &*game;
    let value = &game.get_value();
    match guess.cmp(value) {
        Ordering::Less => CString::new("Too low!").unwrap().into_raw(),
        Ordering::Greater => CString::new("Too high!").unwrap().into_raw(),
        Ordering::Equal => CString::new("Correct!").unwrap().into_raw(),
    }
}

#[no_mangle]
pub unsafe fn destroy_game(game: *mut Game) {
    Box::from_raw(game);
}

// gotta be here to compile
fn main() {}