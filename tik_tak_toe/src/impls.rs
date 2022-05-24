use types::PlayerSymbol;
use std::fmt;

impl fmt::Display for PlayerSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "[{:?}]", self);
    }
}


