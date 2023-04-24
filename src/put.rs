use wasm_bindgen::prelude::wasm_bindgen;

use crate::{enums::COLOR, get_my_and_opponent_stones::get_my_and_opponent_stones};

// Move the place_bit in the specified direction, except for the edge
fn transfer(place: i64, direction: i8) -> i64 {
    match direction {
        0 => ((place << 8) as u64 & 0xffffffffffffff00 as u64) as i64, // up
        1 => ((place << 7) as u64 & 0x7f7f7f7f7f7f7f00 as u64) as i64, // up right
        2 => ((place >> 1) as u64 & 0x7f7f7f7f7f7f7f7f as u64) as i64, // right
        3 => ((place >> 9) as u64 & 0x007f7f7f7f7f7f7f as u64) as i64, // down right
        4 => ((place >> 8) as u64 & 0x00ffffffffffffff as u64) as i64, // down
        5 => ((place >> 7) as u64 & 0x00fefefefefefefe as u64) as i64, // down left
        6 => ((place << 1) as u64 & 0xfefefefefefefefe as u64) as i64, // left
        7 => ((place << 9) as u64 & 0xfefefefefefefe00 as u64) as i64, // up left
        _ => panic!("transfer error."),
    }
}

#[wasm_bindgen]
pub struct PutResult {
    pub black_stones: i64,
    pub white_stones: i64,
    pub put_stones_count: isize,
    pub current_color: isize,
    pub reversed_places: i64,
}

#[wasm_bindgen]
pub fn put(
    black_stones: i64,
    white_stones: i64,
    put_stones_count: isize,
    current_color: isize,
    place: i64,
) -> PutResult {
    let (my_stones, opponent_stones) =
        get_my_and_opponent_stones(black_stones, white_stones, current_color);

    let mut reversed_places: i64 = 0;
    for direction in 0..8 {
        let mut tmp_rev: i64 = 0;
        let mut mask: i64 = transfer(place, direction);
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

    if current_color == COLOR::BLACK as isize {
        let new_black_stones = black_stones | (reversed_places | place);
        let new_white_stones = white_stones & !reversed_places;
        PutResult {
            black_stones: new_black_stones,
            white_stones: new_white_stones,
            put_stones_count: put_stones_count + 1 as isize,
            current_color: COLOR::WHITE as isize,
            reversed_places: reversed_places,
        }
    } else {
        let new_black_stones = black_stones & !reversed_places;
        let new_white_stones = white_stones | (reversed_places | place);
        PutResult {
            black_stones: new_black_stones,
            white_stones: new_white_stones,
            put_stones_count: put_stones_count + 1 as isize,
            current_color: COLOR::BLACK as isize,
            reversed_places: reversed_places,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{enums::COLOR, put::put};

    #[test]
    fn test_put() {
        let black_stones: i64 = 0x002002045c0c0000;
        let white_stones: i64 = 0x0010787820301000;
        let put_stones_count: isize = 22;
        let current_color: isize = COLOR::BLACK as isize;
        let place: i64 = 0x0000000000000008;
        let result = put(
            black_stones,
            white_stones,
            put_stones_count,
            current_color,
            place,
        );
        assert_eq!(result.black_stones, 0x002002045c2c1008);
        assert_eq!(result.white_stones, 0x0010787820100000);
        assert_eq!(result.put_stones_count, 23);
        assert_eq!(result.current_color, COLOR::WHITE as isize);
        println!("{:16x}", result.reversed_places);
        assert_eq!(result.reversed_places, 0x0000000000201000);
    }
}
