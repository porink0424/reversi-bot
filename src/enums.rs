use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum COLOR {
    BLACK,
    WHITE,
}
