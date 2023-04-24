#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum COLOR {
    BLACK,
    WHITE,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ACTION {
    PUT(u64),
    PASS,
}
