use wasm_bindgen::prelude::wasm_bindgen;

use crate::enums::COLOR;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Board {
    pub black_stones: u64,
    pub white_stones: u64,
    pub put_stones_count: usize,
    pub current_color: COLOR,
}

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Board {
        Board {
            black_stones: 0,
            white_stones: 0,
            put_stones_count: 0,
            current_color: COLOR::BLACK,
        }
    }

    pub fn set(
        &mut self,
        black_stones: u64,
        white_stones: u64,
        put_stones_count: usize,
        current_color: COLOR,
    ) {
        self.black_stones = black_stones;
        self.white_stones = white_stones;
        self.put_stones_count = put_stones_count;
        self.current_color = current_color;
    }
}
