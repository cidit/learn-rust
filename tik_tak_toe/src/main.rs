use std::fmt;
use std::io::stdin;

const MAX_SCALE: i32 = 9;
const DEFAULT_SCALE: i32 = 3;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum PlayerSymbol {
    X,
    O,
}

impl fmt::Display for PlayerSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}]", self)
    }
}

type GameState = Vec<Option<PlayerSymbol>>;

// FIXME: should be a tightly encapsulated data structure
struct Game {
    /**
     * the width, height and diagonal of the game
     */
    scale: usize,
    state: GameState,
}

impl Game {
    /**
     * Destructuring function
     */
    fn destruct(&self) -> (usize, &GameState) {
        return (self.scale, &self.state);
    }

    fn destruct_mut(&mut self) -> (usize, &mut GameState) {
        return (self.scale, &mut self.state);
    }

    /**
     * constructor
     * initializes the state according to the specified scale
     */
    fn new(scale: usize) -> Self {
        return Self {
            scale,
            state: (0..scale * scale).map(|_| None).collect(),
        };
    }
}

fn main() {
    println!("Tik Tak Toe!");

    let mut x_points = 0;
    let mut o_points = 0;
    let mut total_games = 0;

    loop {
        let scale = get_game_scale();

        let winner = game_main(scale as usize, PlayerSymbol::O);
        total_games += 1;

        match winner {
            Some(PlayerSymbol::O) => {
                println!("O wins!");
                o_points += 1;
            }
            Some(PlayerSymbol::X) => {
                println!("X wins!");
                x_points += 1;
            }
            None => (),
        };

        println!(
            "[Points] X: {} | O: {} | Total: {}",
            x_points, o_points, total_games
        );

        if !get_continue_game() {
            break;
        }
    }
}

fn get_game_scale() -> i32 {
    loop {
        println!(
            "which scale do you want to play at? [max: {}, default: {}]",
            MAX_SCALE, DEFAULT_SCALE
        );
        let mut input = String::new();
        stdin().read_line(&mut input).expect("fuck");
        match input.trim() {
            "" => return DEFAULT_SCALE,
            other => match other.parse::<i32>() {
                Ok(it) => {
                    if it <= MAX_SCALE && it >= 3 {
                        return it;
                    } else {
                        continue;
                    }
                }
                Err(_) => continue,
            },
        };
    }
}

fn get_continue_game() -> bool {
    loop {
        println!("new game? [y/n, default: y]");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("fuck");
        if input.trim() == "yesn't" {
            println!("which is it god damn it");
            continue;
        }
        match input.trim().get(0..1) {
            Some(input) if input == "y" => return true,
            Some(input) if input == "n" => return false,
            Some(_) => continue,
            None => return true,
        }
    }
}

// FIXME: refactor with result
fn get_next_move(game: &Game) -> (usize, usize) {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let (scale, state) = game.destruct();

    loop {
        println!("enter next move [xy, x: uinteger, y: char]");
        let mut input = String::new();
        stdin().read_line(&mut input).expect("fuck");

        let input: Vec<_> = input.trim().chars().take(2).collect();
        let x = match input.get(0) {
            Some(x) => match x.to_string().parse::<usize>() {
                Ok(x) => x,
                Err(_) => {
                    println!("/!\\ first character is not a number");
                    continue;
                }
            },
            None => {
                println!("/!\\ input is empty");
                continue;
            }
        };
        let y = match input.get(1) {
            Some(y) if y.is_alphabetic() => {
                match alphabet.iter().position(|&c| c as u8 == *y as u8) {
                    Some(y) => y + 1 as usize,
                    None => {
                        println!("/!\\ second character is not a lowercase letter");
                        continue;
                    }
                }
            }
            _ => {
                println!("/!\\ invalid or absent second character");
                continue;
            }
        };

        if 0 == x && x > scale && 0 == y && y > scale {
            println!("/!\\ x or y too big or too small");
            continue;
        }

        let x = x - 1;
        let y = y - 1;

        if state[y * scale + x].is_some() {
            println!("/!\\ theres already something there");
            continue;
        }

        return (x, y);
    }
}

fn game_main(scale: usize, starting_player: PlayerSymbol) -> Option<PlayerSymbol> {
    // TODO: unfinshed
    println!("new game!");

    let mut game = Game::new(scale);

    let mut current_player = starting_player;

    loop {
        println!("Game state:");
        print_game(&game);

        println!("it's {}'s turn to move!", current_player);

        let (x, y) = get_next_move(&game);
        println!();

        let (scale, state) = game.destruct_mut();

        state[y * scale + x] = Some(current_player);

        current_player = if current_player == PlayerSymbol::O {
            PlayerSymbol::X
        } else {
            PlayerSymbol::O
        };

        match who_wins(&game) {
            Some(winner) => return Some(winner),
            None if is_board_full(&game.state) => return None, // thats a draw!
            None => continue,
        }
    }
}

fn print_game(game: &Game) {
    let (scale, state) = game.destruct();
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    println!(
        "\n  {}",
        (1..scale + 1)
            .map(|x| x.to_string())
            .reduce(|a, b| format!("{}{}", a, b))
            .unwrap()
    );
    for x in 0..scale {
        print!("{}|", alphabet[x]);
        // print!("{}", (x + 60) as u8 as char);
        for y in 0..scale {
            print!(
                "{}",
                match state[x * scale + y] {
                    Some(PlayerSymbol::X) => "x",
                    Some(PlayerSymbol::O) => "o",
                    None => ".",
                }
            )
        }
        println!()
    }
    println!();
}

fn who_wins(game: &Game) -> Option<PlayerSymbol> {
    fn all_same<T>(v: &Vec<T>) -> bool
    where
        T: PartialEq,
    {
        return v.windows(2).all(|w| w[0] == w[1]);
    }

    let (scale, state) = game.destruct();

    let first_diagonal: Vec<_> = (0..scale)
        .map(|it| scale * it + it)
        .flat_map(|it| state.get(it))
        .cloned()
        .collect();

    let second_diagonal: Vec<_> = (0..scale)
        .rev()
        .map(|it| it + 1)
        .map(|it| it * scale - it)
        .flat_map(|it| state.get(it))
        .cloned()
        .collect();

    let rows: Vec<Vec<_>> = (0..scale)
        .map(|x| {
            return (0..scale)
                .map(|y| y + scale * x)
                .flat_map(|it| state.get(it))
                .cloned()
                .collect();
        })
        .collect();

    let columns: Vec<_> = (0..scale)
        .map(|x| {
            return (0..scale)
                .map(|y| y * scale + x)
                .flat_map(|it| state.get(it))
                .cloned()
                .collect();
        })
        .collect();

    let mut possible_win_configurations: Vec<Vec<Option<PlayerSymbol>>> = Vec::new();
    possible_win_configurations.push(first_diagonal);
    possible_win_configurations.push(second_diagonal);
    possible_win_configurations.extend(rows);
    possible_win_configurations.extend(columns);

    for configuration in possible_win_configurations {
        if configuration.iter().any(|it| it.is_none()) {
            continue;
        }
        if all_same(&configuration.iter().flatten().collect::<Vec<_>>()) {
            return configuration[0];
        }
    }

    return None;
}

fn is_board_full(state: &GameState) -> bool {
    return state.iter().all(|it| it.is_some());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn who_wins_works() {
        // FIXME: non-exhaustive
        use PlayerSymbol::{O, X};

        let totest = vec![
            Some(X),
            Some(O),
            None,
            Some(X),
            Some(X),
            Some(O),
            None,
            Some(O),
            Some(X),
        ];

        assert_eq!(
            Some(X),
            who_wins(&Game {
                scale: 3,
                state: totest
            })
        );
    }

    #[test]
    fn print_game_works() {
        use PlayerSymbol::{O, X};

        let totest = vec![
            Some(X),
            Some(O),
            None,
            Some(X),
            Some(X),
            Some(O),
            None,
            Some(O),
            Some(X),
        ];

        print_game(&Game {
            scale: 3,
            state: totest,
        })
    }
}
