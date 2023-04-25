use rand::seq::SliceRandom;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{calc_legal_places::calc_legal_places, structs::Board};

#[wasm_bindgen]
pub fn decide_place(board: &Board) -> u64 {
    let legal_places = calc_legal_places(board);
    let mask = 1;
    let mut range = (0..64).collect::<Vec<u8>>();
    range.shuffle(&mut rand::thread_rng());
    for shift in range {
        let randomly_selected_place = (mask << shift) & legal_places;
        if randomly_selected_place != 0 {
            return randomly_selected_place;
        }
    }

    return 0;
}

#[cfg(test)]
mod test {
    use crate::{enums::COLOR, structs::Board};

    #[test]
    fn test_decide_place() {
        let board = Board {
            black_stones: 0x002002045c0c0000,
            white_stones: 0x0010787820301000,
            put_stones_count: 22,
            current_color: COLOR::BLACK,
        };
        println!("decide_place: {:16x}", super::decide_place(&board));
        println!("decide_place: {:16x}", super::decide_place(&board));
        println!("decide_place: {:16x}", super::decide_place(&board));
        println!("decide_place: {:16x}", super::decide_place(&board));
    }
}
