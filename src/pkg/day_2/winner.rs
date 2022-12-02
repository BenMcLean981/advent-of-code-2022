use std::{fmt, str::FromStr};

#[derive(Debug)]
pub(crate) struct WinnerParseError;

impl fmt::Display for WinnerParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Could not read winner.");
    }
}

#[derive(Clone)]
pub(crate) enum Winner {
    Me,
    Them,
    Draw,
}

impl FromStr for Winner {
    type Err = WinnerParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Y" => Ok(Winner::Draw),
            "X" => Ok(Winner::Them),
            "Z" => Ok(Winner::Me),
            _ => Err(WinnerParseError),
        }
    }
}
