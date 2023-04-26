use rand::seq::SliceRandom;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    calc_legal_places::calc_legal_places,
    constants::{EVAL_BY_POINT_TABLE_DEPTH, EVAL_NORMAL_DEPTH, EVAL_PERFECT_DEPTH, EVAL_WIN_DEPTH},
    enums::EvalMethod,
    put::put,
    search::negamax,
    structs::Board,
};

#[wasm_bindgen]
pub fn decide_place(board: &Board, prop_method: EvalMethod) -> u64 {
    match prop_method {
        EvalMethod::Random => {
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
        _ => {
            let limit;
            let method;
            match prop_method {
                EvalMethod::PointTable => {
                    limit = EVAL_BY_POINT_TABLE_DEPTH;
                    method = EvalMethod::PointTable;
                }
                EvalMethod::Normal => {
                    if (64 - board.put_stones_count) as i8 <= EVAL_WIN_DEPTH {
                        if (64 - board.put_stones_count) as i8 <= EVAL_PERFECT_DEPTH {
                            limit = EVAL_PERFECT_DEPTH;
                            method = EvalMethod::Perfect;
                        } else {
                            limit = EVAL_WIN_DEPTH;
                            method = EvalMethod::Win;
                        }
                    } else {
                        limit = EVAL_NORMAL_DEPTH;
                        method = EvalMethod::Normal;
                    }
                }
                _ => unreachable!(),
            };
            let mut place: u64 = 0x0000000000000001;
            let mut max_eval: i32 = std::i32::MIN;
            let mut max_eval_place: u64 = place;
            let legal_places = calc_legal_places(board);
            for _ in 0..64 {
                if place & legal_places != 0 {
                    let put_result = put(board, place);
                    let eval = -negamax(
                        std::i32::MIN + 1,
                        std::i32::MAX - 1,
                        limit - 1,
                        put_result.board,
                        method,
                    );
                    if eval > max_eval {
                        max_eval = eval;
                        max_eval_place = place;
                    }
                };
                place <<= 1;
            }
            return max_eval_place;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        enums::{EvalMethod, COLOR},
        structs::Board,
    };

    #[test]
    fn test_decide_place() {
        let board = Board {
            black_stones: 0x002002045c0c0000,
            white_stones: 0x0010787820301000,
            put_stones_count: 22,
            current_color: COLOR::BLACK,
        };
        println!(
            "decide_place: {:16x}",
            super::decide_place(&board, EvalMethod::Random)
        );
        println!(
            "decide_place: {:16x}",
            super::decide_place(&board, EvalMethod::PointTable)
        );
    }
}
