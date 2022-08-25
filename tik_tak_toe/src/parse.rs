
use crate::constants;

pub fn game_scale(input: &String) -> Result<i32, &'static str> {
    return match input.trim() {
        "" =>  Ok(constants::DEFAULT_SCALE),
        other => match other.parse::<i32>()? {
            scale if scale > constants::MAX_SCALE => Err(format!("'{}' is too big of a scale", scale)),
            scale if scale < 3 => Err(format!("'{}' is too small of a scale", scale)),
            scale => Ok(scale),
        }
    }
}

// pub fn continue(input: &String, accepted_affirmative: &[String], accepted_negative: &[String]) -> Result<bool> {
//     return match input.trim() {
//         "" => Ok(constants::DEFAULT_CONTINUE),
//         other if accepted_affirmative.contains(other) => Ok(true),
//         other if accepted_negative.contains(other) => Ok(false),
//         other => Err(format!("'{}' is not an accepted input", other))
//     }
// }

// pub fn move_numeric(mv: &String) -> Result<(i32, i32)> {
//     let splitted: Vec<_> = match input.trim() {
//         "" => return Err("empty input"),
//         other => other.split(":").collect(),
//     }
//     if splitted.len() != 2 {
//         return Err("input must be two numbers separated by a colon")
//     }
//     let parsed: Vec<_> = splitted
//         .iter()
//         .map(|it| it.parse::<i32>())
//         .map(|it| match it {
//             Ok(it) => it,
//             Err("input must be a pair of valid integer numbers separated by a colon"),
//         })
//         .collect()

//     return (parsed[0], parsed[1])
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_game_scale() {
        // unimplemented!();
    }

    #[test]
    fn test_continue_game() {
        // unimplemented!();
    }

    #[test]
    fn test_move_numeric() {
        // unimplemented!();
    }
}