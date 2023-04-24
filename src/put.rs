use wasm_bindgen::prelude::wasm_bindgen;

use crate::{enums::COLOR, get_my_and_opponent_stones::get_my_and_opponent_stones, structs::Board};

// Move the place_bit in the specified direction, except for the edge
fn transfer(place: u64, direction: u8) -> u64 {
    match direction {
        0 => (place << 8) & 0xffffffffffffff00, // up
        1 => (place << 7) & 0x7f7f7f7f7f7f7f00, // up right
        2 => (place >> 1) & 0x7f7f7f7f7f7f7f7f, // right
        3 => (place >> 9) & 0x007f7f7f7f7f7f7f, // down right
        4 => (place >> 8) & 0x00ffffffffffffff, // down
        5 => (place >> 7) & 0x00fefefefefefefe, // down left
        6 => (place << 1) & 0xfefefefefefefefe, // left
        7 => (place << 9) & 0xfefefefefefefe00, // up left
        _ => panic!("transfer error."),
    }
}

#[wasm_bindgen]
pub struct PutResult {
    pub board: Board,
    pub reversed_places: u64,
}

#[wasm_bindgen]
pub fn put(board: &Board, place: u64) -> PutResult {
    let (my_stones, opponent_stones) = get_my_and_opponent_stones(&board);

    let mut reversed_places: u64 = 0;
    for direction in 0..8 {
        let mut tmp_rev: u64 = 0;
        let mut mask: u64 = transfer(place, direction);
        // Keep updating tmp_rev while opponent's stones continue
        while (mask != 0) && ((mask & opponent_stones) != 0) {
            tmp_rev |= mask;
            mask = transfer(mask, direction);
        }
        // if sandwiched between my stones, add to reversed_places
        if (mask & my_stones) != 0 {
            reversed_places |= tmp_rev;
        }
    }

    if board.current_color == COLOR::BLACK {
        let new_black_stones = board.black_stones | (reversed_places | place);
        let new_white_stones = board.white_stones & !reversed_places;
        PutResult {
            board: Board {
                black_stones: new_black_stones,
                white_stones: new_white_stones,
                put_stones_count: board.put_stones_count + 1,
                current_color: COLOR::WHITE,
            },
            reversed_places: reversed_places,
        }
    } else {
        let new_black_stones = board.black_stones & !reversed_places;
        let new_white_stones = board.white_stones | (reversed_places | place);
        PutResult {
            board: Board {
                black_stones: new_black_stones,
                white_stones: new_white_stones,
                put_stones_count: board.put_stones_count + 1,
                current_color: COLOR::BLACK,
            },
            reversed_places: reversed_places,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{enums::COLOR, put::put};

    #[test]
    fn test_put() {
        let board = crate::structs::Board {
            black_stones: 0x002002045c0c0000,
            white_stones: 0x0010787820301000,
            put_stones_count: 22,
            current_color: COLOR::BLACK,
        };
        let place: u64 = 0x0000000000000008;
        let result = put(&board, place);
        assert_eq!(result.board.black_stones, 0x002002045c2c1008);
        assert_eq!(result.board.white_stones, 0x0010787820100000);
        assert_eq!(result.board.put_stones_count, 23);
        assert_eq!(result.board.current_color, COLOR::WHITE);
        println!("{:16x}", result.reversed_places);
        assert_eq!(result.reversed_places, 0x0000000000201000);
    }
}
