use crate::types::{PlayerSymbol, TikTakToe};
use std::fmt;

impl fmt::Display for PlayerSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "[{:?}]", self);
    }
}

impl fmt::Display for TikTakToe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display = String::new();

        let header = format!(
            "  {}\n",
            (1..self.scale() + 1)
                .map(|x| x.to_string())
                .reduce(|a, b| format!("{}{}", a, b))
                .unwrap()
        );
        display.push_str(&header);

        for x in 0..self.scale() {
            display.push_str(&format!("{}|", x));
            for y in 0..self.scale() {
                use PlayerSymbol::{O, X};
                let str_symbol = match self.state()[x * self.scale() + y] {
                    Some(X) => "x",
                    Some(O) => "o",
                    None => ".",
                };
                display.push_str(str_symbol)
            }
            display.push_str("\n")
        }
        return write!(f, "{}", display);
    }
}