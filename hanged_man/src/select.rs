use itertools::Itertools;
use crate::types::{HangedManState, MoveEvaluation};

pub fn eval_last_move(state: &HangedManState) -> MoveEvaluation {
    unimplemented!()
}

pub fn eval_num_strikes(state: &HangedManState) -> u32 {
    return state.guesses.iter().unique().fold(0, |acc, &item| {
        if !state.word.contains(item) {
            acc + 1
        } else {
            acc
        }
    });
}