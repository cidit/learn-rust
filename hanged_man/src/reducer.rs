
use crate::types::{HangedManAction, HangedManState};

pub fn hanged_man_reducer(state: &HangedManState, action: HangedManAction) -> HangedManState {
    use HangedManAction::*;
    match action {
        Play { guess } => {
            let mut state = state.clone();
            state.guesses.push(guess);
            return state;
        }
    }
}