use crate::types::NoWordReceived;
use reqwest::blocking::get;
use std::error::Error;

pub fn get_random_word() -> Result<String, Box<dyn Error>> {
    let response = get("https://random-words-api.herokuapp.com/w?n=1")?;
    let body: Vec<String> = response.json()?;
    return match body.get(0) {
        Some(s) => Ok(s.clone()),
        None => Err(Box::new(NoWordReceived {})),
    };
}
