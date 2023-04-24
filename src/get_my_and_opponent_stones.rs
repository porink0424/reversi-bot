use crate::enums::COLOR;

pub fn get_my_and_opponent_stones(
    black_stones: i64,
    white_stones: i64,
    current_color: isize,
) -> (i64, i64) {
    let my_stones: i64;
    let opponent_stones: i64;
    if current_color == COLOR::BLACK as isize {
        my_stones = black_stones;
        opponent_stones = white_stones;
    } else {
        my_stones = white_stones;
        opponent_stones = black_stones;
    }
    return (my_stones, opponent_stones);
}
