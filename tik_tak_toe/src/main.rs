#[derive(Debug)]
enum PlayerSymbol {
    X,
    O,
}

const MAX_SCALE: i32 = 9;

fn main() {
    println!("Tik Tak Toe!");

    let scale = 3;
    let alphabet: Vec<_> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let places: Vec<Option<PlayerSymbol>> = (0..scale * scale).map(|_| None).collect();

    println!(
        " {}",
        (1..scale + 1).map(|x| x.to_string()).reduce(|a, b| format!("{}{}", a, b)).unwrap()
    );


    for x in 0..scale {
        print!("{}", alphabet[x]);
        for y in 0..scale {
            print!(
                "{}",
                match &places[x * y] {
                    Some(PlayerSymbol::X) => "x",
                    Some(PlayerSymbol::O) => "o",
                    None => ".",
                }
            )
        }
        println!()
    }
}
