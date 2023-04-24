use crate::enums::COLOR;

pub struct Board {
    pub black_stones: i64,
    pub white_stones: i64,
    pub put_stones_count: isize,
    pub current_color: COLOR,
}
