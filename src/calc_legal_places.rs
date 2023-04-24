use wasm_bindgen::prelude::wasm_bindgen;

use crate::{get_my_and_opponent_stones::get_my_and_opponent_stones, structs::Board};

#[wasm_bindgen]
pub fn calc_legal_places(board: &Board) -> u64 {
    let (my_stones, opponent_stones) = get_my_and_opponent_stones(board);

    let vertical_zeros: u64 = opponent_stones & 0x7e7e7e7e7e7e7e7e;
    let horizontal_zeros: u64 = opponent_stones & 0x00FFFFFFFFFFFF00;
    let all_edge_zeros: u64 = opponent_stones & 0x007e7e7e7e7e7e00;
    let blank_places: u64 = !(my_stones | opponent_stones);
    let mut tmp: u64;
    let mut legal_places: u64;

    // Stand the bits where it can sandwitch opponent's stones from the left side
    tmp = vertical_zeros & (my_stones << 1);
    for _ in 0..5 {
        tmp |= vertical_zeros & (tmp << 1)
    }
    legal_places = blank_places & (tmp << 1);

    // from the right side
    tmp = vertical_zeros & (my_stones >> 1);
    for _ in 0..5 {
        tmp |= vertical_zeros & (tmp >> 1)
    }
    legal_places |= blank_places & (tmp >> 1);

    // from the top
    tmp = horizontal_zeros & (my_stones << 8);
    for _ in 0..5 {
        tmp |= horizontal_zeros & (tmp << 8)
    }
    legal_places |= blank_places & (tmp << 8);

    // from the bottom
    tmp = horizontal_zeros & (my_stones >> 8);
    for _ in 0..5 {
        tmp |= horizontal_zeros & (tmp >> 8)
    }
    legal_places |= blank_places & (tmp >> 8);

    // from the right top
    tmp = all_edge_zeros & (my_stones << 7);
    for _ in 0..5 {
        tmp |= all_edge_zeros & (tmp << 7)
    }
    legal_places |= blank_places & (tmp << 7);

    // from the left top
    tmp = all_edge_zeros & (my_stones << 9);
    for _ in 0..5 {
        tmp |= all_edge_zeros & (tmp << 9)
    }
    legal_places |= blank_places & (tmp << 9);

    // from the right bottom
    tmp = all_edge_zeros & (my_stones >> 9);
    for _ in 0..5 {
        tmp |= all_edge_zeros & (tmp >> 9)
    }
    legal_places |= blank_places & (tmp >> 9);

    // from the left bottom
    tmp = all_edge_zeros & (my_stones >> 7);
    for _ in 0..5 {
        tmp |= all_edge_zeros & (tmp >> 7)
    }
    legal_places |= blank_places & (tmp >> 7);

    return legal_places;
}

#[cfg(test)]
mod tests {
    use crate::{calc_legal_places::calc_legal_places, enums::COLOR, structs::Board};

    #[test]
    fn test_calc_legal_places() {
        let board = Board {
            black_stones: 0x0000001008000000,
            white_stones: 0x0000000810000000,
            put_stones_count: 4,
            current_color: COLOR::BLACK,
        };
        let legal_places = calc_legal_places(&board);
        assert_eq!(legal_places, 0x0000080420100000);

        let board = Board {
            black_stones: 0x0000001008000000,
            white_stones: 0x0000000810000000,
            put_stones_count: 4,
            current_color: COLOR::WHITE,
        };
        let legal_places = calc_legal_places(&board);
        assert_eq!(legal_places, 0x0000102004080000);

        let board = Board {
            black_stones: 0x002002045c0c0000,
            white_stones: 0x0010787820301000,
            put_stones_count: 22,
            current_color: COLOR::BLACK,
        };
        let legal_places = calc_legal_places(&board);
        assert_eq!(legal_places, 0x30c8048000406038);
    }
}
