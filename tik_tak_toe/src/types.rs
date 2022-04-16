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
pub enum GameInitErrKind {
    ScaleSmallerThanStroke,
    IncoherentMoves {
        move_count: u32,
        reason: MoveValidity,
    },
}

#[derive(Debug)]
pub enum MoveValidity {
    Valid,
    OutOfBounds,
    AlreadyTaken,
}

pub type GameState = Vec<Option<PlayerSymbol>>;

// FIXME: should be a tightly encapsulated data structure
pub struct TikTakToe {
    scale: usize,
    state: GameState,
    stroke: u32,
    current_player: PlayerSymbol,
}

impl TikTakToe {

    pub fn scale(&self) -> usize {
        return self.scale;
    }

    pub fn state(&self) -> &GameState {
        return &self.state;
    }

    pub fn stroke(&self) -> u32 {
        return self.stroke;
    }

    pub fn current_player(&self) -> PlayerSymbol {
        return self.current_player;
    }
 
    /**
     * constructor
     * initializes an empty state according of the specified scale
     */
    pub fn new(
        scale: usize,
        stroke: u32,
        starting_player: PlayerSymbol,
    ) -> Result<Self, GameInitErrKind> {
        use GameInitErrKind::ScaleSmallerThanStroke;

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

    /**
     * constructor
     * populate the state according to the specified list of moves
     */
    pub fn from(
        scale: usize,
        stroke: u32,
        starting_player: PlayerSymbol,
        moves: Vec<(i32, i32)>,
    ) -> Result<Self, GameInitErrKind> {
        use GameInitErrKind::IncoherentMoves;

        let game = Self::new(scale, stroke, starting_player)?;

        for (i, mv) in moves.iter().enumerate() {
            match game.is_move_valid(mv) {
                Valid => {
                    game.play(mv);
                }
                reason => {
                    return Err(IncoherentMoves {
                        move_count: i as u32,
                        reason,
                    });
                }
            }
        }

        return Ok(game);
    }

    pub fn winner(&self) -> Option<PlayerSymbol> {
        let bound = self.scale as i32;

        let generate_coords = |x: i32, a: i32, b: i32| (x, x * a + b);
        let is_in_bounds = |(_x, y): &(i32, i32)| *y <= bound && *y >= 0;
        let find_in_state =
            |(x, y): (i32, i32)| self.state.get(x as usize * self.scale + y as usize);

        let pos_a_diag: Vec<Vec<_>> = (bound..-bound)
            .map(|b| {
                (0..bound)
                    .map(|x| generate_coords(x, 1, b))
                    .filter(is_in_bounds)
                    .flat_map(find_in_state)
                    .cloned()
                    .collect()
            })
            .collect();

        let neg_a_diag: Vec<Vec<_>> = (0..2 * bound)
            .map(|b| {
                (0..bound)
                    .map(|x| generate_coords(x, 1, b))
                    .filter(is_in_bounds)
                    .flat_map(find_in_state)
                    .cloned()
                    .collect()
            })
            .collect();

        let rows: Vec<Vec<_>> = (0..bound)
            .map(|x| {
                (0..bound)
                    .map(|y| (x, y))
                    .flat_map(find_in_state)
                    .cloned()
                    .collect()
            })
            .collect();

        let columns: Vec<Vec<_>> = (0..bound)
            .map(|x| {
                (0..bound)
                    .map(|y| (y, x))
                    .flat_map(find_in_state)
                    .cloned()
                    .collect()
            })
            .collect();

        let lines = [pos_a_diag, neg_a_diag, columns, rows];
        let lines: Vec<_> = Vec::from(lines).iter().flatten().collect();

        let all_some = |v: &Vec<_>| v.iter().all(Option::is_some);
        let all_same = |v: &Vec<_>| v.windows(2).all(|w| w[0] == w[1]);

        for line in lines {
            let possible_win_configurations: Vec<PlayerSymbol> = line
                .windows(self.stroke as usize)
                .map(Vec::from)
                .filter(all_some)
                .filter(all_same)
                .map(|v| v.get(0).unwrap())
                .flatten()
                .cloned()
                .collect();
            if possible_win_configurations.len() > 0 {
                return possible_win_configurations.get(0).map(|winner| *winner);
            }
        }

        return None;
    }

    fn is_board_full(&self) -> bool {
        return self.state.iter().all(|it| it.is_some());
    }

    /**
     * may trigger a side effect
     */
    pub fn play(&self, player_move: &(i32, i32)) -> MoveValidity {
        let (x, y) = player_move;

        match self.is_move_valid(player_move) {
            Valid => {
                use PlayerSymbol::*;
                self.apply_move(*x as usize, *y as usize);
                self.current_player = if self.current_player == O { X } else { O };
                return Valid;
            }
            reason => return reason,
        }
    }

    fn is_move_valid(&self, player_move: &(i32, i32)) -> MoveValidity {
        use MoveValidity::*;

        let (x, y) = *player_move;

        if x < 0 || y < 0 || x > self.scale as i32 || y > self.scale as i32 {
            return OutOfBounds;
        }

        let (x, y) = (x as usize, y as usize);
        if self.state[y * self.scale + x].is_some() {
            return AlreadyTaken;
        }

        return Valid;
    }

    /**
     * Applies side effect
     */
    fn apply_move(&self, x: usize, y: usize) {
        self.state[y * self.scale + x] = Some(self.current_player);
    }
}

impl fmt::Display for TikTakToe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
            display.push_str(&format!("{}|", x));
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
        let game = TikTakToe::new(3, 3, PlayerSymbol::X).unwrap();
        let actual = format!("{}", game);
        let expected = concat!("  123\n", "a|...\n", "b|...\n", "c|...\n",);
        assert_eq!(expected, actual)
    }
}
