/* 
trying out the reducer pattern for handling out the logic of turn based game development
*/

use std::vec;


#[derive(Clone)]
pub struct HangedManState {
    word: String,
    guesses: Vec<char>,
}

pub enum HangedManAction {
    Play { guess: char }
}

pub fn hanged_man_reducer(state: &HangedManState, action: HangedManAction) -> HangedManState {
    use HangedManAction::*;
    match action {
        Play { guess } => {
            let mut state = state.clone();
            state.guesses.push(guess);

            return state
        }
    }
}
