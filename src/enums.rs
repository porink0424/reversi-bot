use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum COLOR {
    BLACK,
    WHITE,
}

#[wasm_bindgen]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum EvalMethod {
    Random,
    PointTable, // using point table, mainly for debug
    Normal,
    Win,     // complete search
    Perfect, // complete search (also care about the number of pieces)
}

#[wasm_bindgen]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum WinPrediction {
    WIN,
    LOSE,
    DRAW,
    UNKNOWN,
}
