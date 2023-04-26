use crate::{get_my_and_opponent_stones::get_my_and_opponent_stones, structs::Board};

pub fn evaluate_perfect(board: Board) -> i32 {
    let (my_stones, opponent_stones) = get_my_and_opponent_stones(&board);
    return my_stones.count_ones() as i32 - opponent_stones.count_ones() as i32;
}
