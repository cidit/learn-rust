use crate::types::{HangedManState, MoveEvaluation};
use itertools::Itertools;

pub fn eval_last_move(state: &HangedManState) -> MoveEvaluation {
    use MoveEvaluation::*;

    let last = match state.guesses.last() {
        Some(&character) => character,
        None => return NoMoves,
    };

    if state
        .guesses
        .iter()
        .dropping_back(1)
        .any(|&item| item == last)
    {
        return Repeating;
    }

    return if state.word.contains(last) {
        GoodMove
    } else {
        BadMove
    };
}

pub fn num_strikes(state: &HangedManState) -> u32 {
    state.guesses.iter().unique().fold(0, |acc, &item| {
        if !state.word.contains(item) {
            acc + 1
        } else {
            acc
        }
    })
}

pub fn word_guessed(state: &HangedManState) -> bool {
    state
        .word
        .chars()
        .all(|c| state.guesses.iter().any(|&g| g == c))
}
