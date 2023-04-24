#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum COLOR {
    BLACK = 1 as isize,
    WHITE = -1 as isize,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ACTION {
    PUT(i64),
    PASS,
}
