pub mod model;

pub fn parse(input: &str) -> Vec<model::Token> {
    unimplemented!();
}

pub fn evaluate(tokens: &[model::Token]) 
-> Result<num::BigRational, Box<dyn std::error::Error>> 
{
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        unimplemented!();
    }
}