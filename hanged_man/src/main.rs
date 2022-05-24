use hanged_man::misc::get_random_word;
use hanged_man::types::{HangedManState, HangedManAction};

fn main() {
    let word = get_random_word();
    println!("{}", word.ok().unwrap());


    loop {
        let word = get_random_word().ok().expect("Failed to fetch word");
        let game_state = HangedManState::new(&word);

    }
}
