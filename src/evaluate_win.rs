use crate::{get_my_and_opponent_stones::get_my_and_opponent_stones, structs::Board};

pub fn evaluate_win(board: Board) -> i32 {
    let (my_stones, opponent_stones) = get_my_and_opponent_stones(&board);
    let diff = my_stones.count_ones() as i32 - opponent_stones.count_ones() as i32;
    if diff > 0 {
        1
    } else if diff == 0 {
        0
    } else {
        -1
    }
}
