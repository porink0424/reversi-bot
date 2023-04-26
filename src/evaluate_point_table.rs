use crate::{get_my_and_opponent_stones::get_my_and_opponent_stones, structs::Board};

pub fn evaluate_point_table(board: Board) -> i32 {
    let mut point: i32 = 0;
    let (my_stones, opponent_stones) = get_my_and_opponent_stones(&board);
    point += (my_stones & 0x8100000000000081).count_ones() as i32 * (100); // A1
    point += (my_stones & 0x4281000000008142).count_ones() as i32 * (-50); // B1
    point += (my_stones & 0x2400810000810024).count_ones() as i32 * (10); // C1
    point += (my_stones & 0x0042000000004200).count_ones() as i32 * (-70); // B2
    point += (my_stones & 0x0024420000422400).count_ones() as i32 * (-5); // C2
    point += (my_stones & 0x0018244242241800).count_ones() as i32 * (-10); // D2,C3
    point += (my_stones & 0x0000182424180000).count_ones() as i32 * (-5); // D3
    point -= (opponent_stones & 0x8100000000000081).count_ones() as i32 * (100); // A1
    point -= (opponent_stones & 0x4281000000008142).count_ones() as i32 * (-50); // B1
    point -= (opponent_stones & 0x2400810000810024).count_ones() as i32 * (10); // C1
    point -= (opponent_stones & 0x0042000000004200).count_ones() as i32 * (-70); // B2
    point -= (opponent_stones & 0x0024420000422400).count_ones() as i32 * (-5); // C2
    point -= (opponent_stones & 0x0018244242241800).count_ones() as i32 * (-10); // D2,C3
    point -= (opponent_stones & 0x0000182424180000).count_ones() as i32 * (-5); // D3
    return point;
}
