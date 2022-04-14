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

#[derive(Debug)]
pub enum GameInitErr {
    ScaleSmallerThanStroke,
}

pub type GameState = Vec<Option<PlayerSymbol>>;

// FIXME: should be a tightly encapsulated data structure
pub struct Game {
    scale: usize,
    state: GameState,
    stroke: u16,
    current_player: PlayerSymbol,
}

impl Game {
    /**
     * constructor
     * initializes the state according to the specified scale
     */
    pub fn new(
        scale: usize,
        stroke: u16,
        starting_player: PlayerSymbol,
    ) -> Result<Self, GameInitErr> {
        use GameInitErr::ScaleSmallerThanStroke;

        if scale < stroke as usize {
            return Err(ScaleSmallerThanStroke);
        }

        return Ok(Self {
            scale,
            state: (0..scale * scale).map(|_| None).collect(),
            stroke,
            current_player: starting_player,
        });
    }

    pub fn from(
        scale: usize,
        stroke: u16,
        starting_player: PlayerSymbol,
        saved: Vec<(i32, char)>
    ) -> Result<Self, GameInitErr> {

    }

    pub fn winner() -> Option<PlayerSymbol> {
        unimplemented!()
    }

    pub fn finished() -> bool {
        unimplemented!()
    }

    fn is_board_full(&self) -> bool {
        return self.state.iter().all(|it| it.is_some());
    }

    pub fn make_move(player_move: (i32, char)) {
        unimplemented!()
    }

    fn apply_move(&self, x: usize, y: usize) {
        self.state[y * self.scale + x] = Some(self.current_player);
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
    fn game_creation() {
        unimplemented!();
        // testing constructors
    }

    #[test]
    fn fmt_game() {
        let game = Game::new(3, 3, PlayerSymbol::X).unwrap();
        let actual = format!("{}", game);
        let expected = concat!("  123\n", "a|...\n", "b|...\n", "c|...\n",);
        assert_eq!(expected, actual)
    }
}
