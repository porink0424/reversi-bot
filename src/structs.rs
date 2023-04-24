use crate::enums::COLOR;

pub struct Board {
    pub black_stones: u64,
    pub white_stones: u64,
    pub put_stones_count: usize,
    pub current_color: COLOR,
}
