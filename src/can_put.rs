use wasm_bindgen::prelude::wasm_bindgen;

use crate::{calc_legal_places::calc_legal_places, structs::Board};

#[wasm_bindgen]
pub fn can_put(board: &Board, place: u64) -> bool {
    let legal_places = calc_legal_places(board);
    return (place & legal_places) == place;
}
