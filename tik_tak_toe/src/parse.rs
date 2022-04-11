mod constants;
pub fn parse_game_scale(input: String) -> Result<i32> {
    match input.trim() {
        "" =>  constants::DEFAULT_SCALE,
    }
}