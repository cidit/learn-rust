use std::io::stdin;

const MAX_SCALE: i32 = 9;
const DEFAULT_SCALE: i32 = 3;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum PlayerSymbol {
    X,
    O,
}

type GameState = Vec<Option<PlayerSymbol>>;

struct Game {
    state: GameState,
    scale: usize,
}

impl Game {
    /**
     * Destructuring function
     */
    fn destruct(self: &Self) -> (usize, &GameState) {
        return (self.scale, &self.state);
    }
}

fn main() {
    println!("Tik Tak Toe!");

    loop {
        let scale = get_game_scale();

        game_main(scale as usize);

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
    // FIXME: not taking into account anything other than "n" or ""
    // TODO: add "yesn't" easter egg?

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

fn game_main(scale: usize) {
    // TODO: unfinshed
    println!("new game!");

    let game = Game {
        scale,
        state: (0..scale * scale).map(|_| None).collect(),
    };

    loop {
        println!("Game state:");
        print_game(&game);

        who_wins(&game);
        loop {}
    }
}

fn print_game(game: &Game) {
    let (scale, state) = game.destruct();
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    println!(
        " {}",
        (1..scale + 1)
            .map(|x| x.to_string())
            .reduce(|a, b| format!("{}{}", a, b))
            .unwrap()
    );
    for x in 0..scale {
        print!("{}", alphabet[x]);
        for y in 0..scale {
            print!(
                "{}",
                match state[x * y] {
                    Some(PlayerSymbol::X) => "x",
                    Some(PlayerSymbol::O) => "o",
                    None => ".",
                }
            )
        }
        println!()
    }
}

// FIXME: try to remove the ref on the Option<&PlayerSymbol>
// TODO: test that shit
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
}
