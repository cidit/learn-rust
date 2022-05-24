const endpoint: &str = "https://random-words-api.herokuapp.com/w?n=1";

fn main() {
    let body: Vec<&str> = reqwest::blocking::get(endpoint)?.json()?;
    
}
