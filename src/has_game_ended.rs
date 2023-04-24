use wasm_bindgen::prelude::wasm_bindgen;

use crate::{calc_legal_places::calc_legal_places, enums::COLOR, structs::Board};

#[wasm_bindgen]
pub fn has_game_ended(board: &Board) -> bool {
    let mut tmp_board = Board {
        black_stones: board.black_stones,
        white_stones: board.white_stones,
        put_stones_count: board.put_stones_count,
        current_color: COLOR::BLACK,
    };
    let black_legal_places = calc_legal_places(&tmp_board);

    tmp_board.current_color = COLOR::WHITE;
    let white_legal_places = calc_legal_places(&tmp_board);

    return black_legal_places == 0 && white_legal_places == 0;
}

#[cfg(test)]
mod test {
    use crate::{enums::COLOR, structs::Board};

    #[test]
    fn test_has_game_ended() {
        let board = Board {
            black_stones: 0x7050700000000000,
            white_stones: 0x0020000000000000,
            put_stones_count: 9,
            current_color: COLOR::BLACK,
        };
        assert_eq!(super::has_game_ended(&board), false);

        let board = Board {
            black_stones: 0x7050700000000000,
            white_stones: 0x0020000000000000,
            put_stones_count: 9,
            current_color: COLOR::WHITE,
        };
        assert_eq!(super::has_game_ended(&board), false);

        let board = Board {
            black_stones: 0x7070700000000000,
            white_stones: 0x0000000000000000,
            put_stones_count: 9,
            current_color: COLOR::BLACK,
        };
        assert_eq!(super::has_game_ended(&board), true);

        let board = Board {
            black_stones: 0x7070700000000000,
            white_stones: 0x0000000000000000,
            put_stones_count: 9,
            current_color: COLOR::WHITE,
        };
        assert_eq!(super::has_game_ended(&board), true);
    }
}
