use crate::{
    enums::EvalMethod, evaluate_normal::evaluate_normal, evaluate_perfect::evaluate_perfect,
    evaluate_point_table::evaluate_point_table, evaluate_win::evaluate_win, structs::Board,
};

pub fn evaluate(method: EvalMethod, board: Board) -> i32 {
    match method {
        EvalMethod::Random => unreachable!(),
        EvalMethod::PointTable => evaluate_point_table(board),
        EvalMethod::Normal => evaluate_normal(board),
        EvalMethod::Perfect => evaluate_perfect(board),
        EvalMethod::Win => evaluate_win(board),
    }
}
