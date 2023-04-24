use crate::{enums::COLOR, structs::Board};

pub fn get_my_and_opponent_stones(board: &Board) -> (u64, u64) {
    let my_stones: u64;
    let opponent_stones: u64;
    if board.current_color == COLOR::BLACK {
        my_stones = board.black_stones;
        opponent_stones = board.white_stones;
    } else {
        my_stones = board.white_stones;
        opponent_stones = board.black_stones;
    }
    return (my_stones, opponent_stones);
}
