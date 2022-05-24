use hanged_man::misc::get_random_word;

fn main() {
    let word = get_random_word();
    println!("{}", word.ok().unwrap())
}
