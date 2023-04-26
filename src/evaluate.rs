use crate::{enums::EvalMethod, evaluate_point_table::evaluate_point_table, structs::Board};

pub fn evaluate(method: EvalMethod, board: Board) -> i32 {
    match method {
        EvalMethod::Random => unreachable!(),
        EvalMethod::PointTable => evaluate_point_table(board),
        EvalMethod::Normal => todo!(),
        EvalMethod::Perfect => todo!(),
        EvalMethod::WinOrLose => todo!(),
    }
}
