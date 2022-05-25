use hanged_man::misc::get_random_word;
use hanged_man::types::{HangedManAction, HangedManState};
use hanged_man::reducer::hanged_man_reducer;
use hanged_man::select;
use itertools::Itertools;

fn main() {

    'game: loop {
        let word = get_random_word().ok().expect("Failed to fetch word");
        let mut game_state = HangedManState::new(&word);
        'turn: loop {
            let formatted = game_state
                .word()
                .chars()
                .map(|c| {
                    if game_state.guesses().iter().any(|&g| g == c) {
                        c
                    } else {
                        '_'
                    }
                })
                .intersperse(' ')
                .collect::<String>();

            println!("{}", formatted);

            println!("please guess a letter");
            
            let mut guess = String::new();
            std::io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess = match guess.trim().chars().nth(0) {
                Some(s) => s,
                None => {
                    println!("input needed");
                    continue 'turn;
                }
            };

            game_state = hanged_man_reducer(&game_state, HangedManAction::Play { guess });

            if select::word_guessed(&game_state) {
                println!("youre right!");
                break 'turn;
            }
        }
    }
}
