// depths for each evaluation
pub const EVAL_BY_POINT_TABLE_DEPTH: i8 = 8;
pub const EVAL_NORMAL_DEPTH: i8 = 8;
pub const EVAL_PERFECT_DEPTH: i8 = 16;
pub const EVAL_WIN_DEPTH: i8 = 18;

// weights used in normal evaluation
pub const WEIGHT_STABLE: i32 = 100;
pub const WEIGHT_WING: i32 = -310;
pub const WEIGHT_XMOVE: i32 = -450;
pub const WEIGHT_CMOVE: i32 = -550;
pub const WEIGHT_MOBILITY: i32 = 135;
pub const WEIGHT_OPENNESS: i32 = -10;
