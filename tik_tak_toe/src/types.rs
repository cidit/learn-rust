use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PlayerSymbol {
    X,
    O,
}

impl fmt::Display for PlayerSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "[{:?}]", self);
    }
}

pub type GameState = Vec<Option<PlayerSymbol>>;

// FIXME: should be a tightly encapsulated data structure
pub struct Game {
    /**
     * the width, height and diagonal of the game
     */
    pub scale: usize,
    pub state: GameState,
}

impl Game {
    /**
     * Destructuring function
     */
    pub fn destruct(&self) -> (usize, &GameState) {
        return (self.scale, &self.state);
    }

    pub fn destruct_mut(&mut self) -> (usize, &mut GameState) {
        return (self.scale, &mut self.state);
    }

    /**
     * constructor
     * initializes the state according to the specified scale
     */
    pub fn new(scale: usize) -> Self {
        return Self {
            scale,
            state: (0..scale * scale).map(|_| None).collect(),
        };
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        let mut display = String::new();

        let header = format!(
            "  {}\n",
            (1..self.scale + 1)
                .map(|x| x.to_string())
                .reduce(|a, b| format!("{}{}", a, b))
                .unwrap()
        );
        display.push_str(&header);

        for x in 0..self.scale {
            display.push_str(&format!("{}|", alphabet[x]));
            for y in 0..self.scale {
                use PlayerSymbol::{O, X};
                let str_symbol = match self.state[x * self.scale + y] {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt_game() {
        let game = Game::new(3);
        let actual = format!("{}", game);
        let expected = concat!("  123\n", "a|...\n", "b|...\n", "c|...\n",);
        assert_eq!(expected, actual)
    }
}
