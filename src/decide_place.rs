use rand::seq::SliceRandom;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    calc_legal_places::calc_legal_places,
    constants::{EVAL_BY_POINT_TABLE_DEPTH, EVAL_NORMAL_DEPTH, EVAL_PERFECT_DEPTH, EVAL_WIN_DEPTH},
    enums::{EvalMethod, WinPrediction},
    put::put,
    search::negamax,
    structs::Board,
};

#[wasm_bindgen]
#[derive(Debug)]
pub struct DecidePlaceResult {
    pub place: u64,
    pub win_prediction: WinPrediction,
}

#[wasm_bindgen]
pub fn decide_place(board: &Board, prop_method: EvalMethod) -> DecidePlaceResult {
    match prop_method {
        EvalMethod::Random => {
            let legal_places = calc_legal_places(board);
            let mask = 1;
            let mut range = (0..64).collect::<Vec<u8>>();
            range.shuffle(&mut rand::thread_rng());
            for shift in range {
                let randomly_selected_place = (mask << shift) & legal_places;
                if randomly_selected_place != 0 {
                    return DecidePlaceResult {
                        place: randomly_selected_place,
                        win_prediction: WinPrediction::UNKNOWN,
                    };
                }
            }
            return DecidePlaceResult {
                place: 0,
                win_prediction: WinPrediction::UNKNOWN,
            };
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

            // return with win prediction
            match method {
                EvalMethod::Win | EvalMethod::Perfect => {
                    if max_eval > 0 {
                        return DecidePlaceResult {
                            place: max_eval_place,
                            win_prediction: WinPrediction::WIN,
                        };
                    } else if max_eval == 0 {
                        return DecidePlaceResult {
                            place: max_eval_place,
                            win_prediction: WinPrediction::DRAW,
                        };
                    } else {
                        return DecidePlaceResult {
                            place: max_eval_place,
                            win_prediction: WinPrediction::LOSE,
                        };
                    }
                }
                _ => {
                    return DecidePlaceResult {
                        place: max_eval_place,
                        win_prediction: WinPrediction::UNKNOWN,
                    };
                }
            }
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
            black_stones: 0x007fe8c8bfe3f53e,
            white_stones: 0xfc001030401c0800,
            put_stones_count: 51,
            current_color: COLOR::WHITE,
        };
        println!(
            "decide_place: {:?}",
            super::decide_place(&board, EvalMethod::Normal)
        );

        let board = Board {
            black_stones: 0x818080000,
            white_stones: 0x1000000000,
            put_stones_count: 5,
            current_color: COLOR::WHITE,
        };
        println!(
            "decide_place: {:?}",
            super::decide_place(&board, EvalMethod::Normal)
        );
    }
}
