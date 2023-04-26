use std::cmp::max;

use crate::{
    calc_legal_places::calc_legal_places, enums::EvalMethod, evaluate::evaluate,
    has_game_ended::has_game_ended, put::put, structs::Board, utils::swap_board,
};

// search with alpha-beta method
pub fn negamax(
    prop_alpha: i32,
    prop_beta: i32,
    limit: i8,
    mut board: Board,
    method: EvalMethod,
) -> i32 {
    let mut alpha = prop_alpha;
    let beta = prop_beta;

    if limit == 0 || has_game_ended(&board) {
        return evaluate(method, board);
    }

    let legal_plalces = calc_legal_places(&board);
    let mut score: i32;

    if legal_plalces.count_ones() == 0 as u32 {
        // pass
        swap_board(&mut board);
        score = -negamax(-beta, -alpha, limit, board, method); // preceed to search with same limit
        swap_board(&mut board);
        return score;
    }

    let mut score_max = std::i32::MIN;
    let mut mask: u64 = 0x0000000000000001;
    for _ in 0..64 {
        if mask & legal_plalces != 0 {
            // mask is the place where we can put
            let put_result = put(&board, mask);
            score = -negamax(-beta, -alpha, limit - 1, put_result.board, method);
            if score > beta {
                // beta cut
                return score;
            }
            if score > score_max {
                score_max = score;
                alpha = max(alpha, score_max);
            }
        }
        mask <<= 1;
    }

    score_max
}
