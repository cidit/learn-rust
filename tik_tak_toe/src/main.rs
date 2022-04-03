use std::io::stdin;

const MAX_SCALE: i32 = 9;
const DEFAULT_SCALE: i32 = 3;

#[derive(Debug)]
enum PlayerSymbol {
    X,
    O,
}

fn main() {
    println!("Tik Tak Toe!");

    'main: loop {

        let scale = get_game_scale();

        game_main(scale as usize);


        'exit: loop {
            println!("new game? [y/n]");
            let mut input = String::new();
            stdin().read_line(&mut input).expect("fuck");
            match input.trim().get(0..1) {
                Some(input) => if "n".to_owned() == input { break 'main } else { continue 'main},
                None => continue 'exit,
            }
        }
    }

}

fn get_game_scale() -> i32 {
    loop {
        println!("which scale do you want to play at? [max: {}, default: {}]", MAX_SCALE, DEFAULT_SCALE);
        let mut input = String::new();
        stdin().read_line(&mut input).expect("fuck");
        match input.trim() {
            "" => return DEFAULT_SCALE,
            other => match other.parse::<i32>() {
                Ok(it) => if it <= MAX_SCALE && it >= 1 { return it } else { continue },
                Err(_) => continue,
            },
        };
    }
}

fn game_main(scale: usize) {
    // TODO: unfinshed
    println!("new game!");

    let places: Vec<Option<PlayerSymbol>> = (0..scale * scale).map(|_| None).collect();

    loop {}
}

fn print_game(scale: usize, game_state: Vec<Option<PlayerSymbol>>) {

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
                match &game_state[x * y] {
                    Some(PlayerSymbol::X) => "x",
                    Some(PlayerSymbol::O) => "o",
                    None => ".",
                }
            )
        }
        println!()
    }
}