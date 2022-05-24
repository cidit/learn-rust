#[derive(Clone)]
pub struct HangedManState {
    pub(crate) word: String,
    pub(crate) guesses: Vec<char>,
}

pub enum HangedManAction {
    Play { guess: char },
}

pub enum MoveEvaluation {
    GoodMove,
    BadMove,
    Repeating,
}