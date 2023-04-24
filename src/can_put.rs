use wasm_bindgen::prelude::wasm_bindgen;

use crate::calc_legal_places::calc_legal_places;

#[wasm_bindgen]
pub fn can_put(black_stones: i64, white_stones: i64, current_color: isize, place: i64) -> bool {
    let legal_places = calc_legal_places(black_stones, white_stones, current_color);
    return (place & legal_places) == place;
}
