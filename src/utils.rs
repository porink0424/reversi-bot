use crate::{enums::COLOR, structs::Board};

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn swap_board(board: &mut Board) {
    board.current_color = if COLOR::BLACK == board.current_color {
        COLOR::WHITE
    } else {
        COLOR::BLACK
    };
}
