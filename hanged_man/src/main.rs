const endpoint: &str = "https://random-words-api.herokuapp.com/w?n=1";

fn main() {
    let response = reqwest::blocking::get(endpoint).unwrap();
    let word: Vec<String> = response.json().unwrap();
}
