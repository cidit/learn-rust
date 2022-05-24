use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Clone)]
pub struct HangedManState {
    pub(crate) word: String,
    pub(crate) guesses: Vec<char>,
}

impl HangedManState {
    pub fn new(word: &str) -> Self {
        Self { word: word.to_string(), guesses: vec![] }
    }
}

pub enum HangedManAction {
    Play { guess: char },
}

pub enum MoveEvaluation {
    GoodMove,
    BadMove,
    Repeating,
    NoMoves,
}

#[derive(Debug)]
pub struct NoWordReceived;

impl Error for NoWordReceived {}

impl Display for NoWordReceived {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "expected smth, got nothin")
    }
}
