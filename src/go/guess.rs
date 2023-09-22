use core::convert::From;
use std::os::raw::c_uchar;

use crate::go::string::GoString;

#[repr(C)]
pub struct GoGuess {
    language: GoString,
    safe: c_uchar,
}


/// The result of a language guess.
#[derive(Debug)]
pub struct Guess {
    /// Which language it is
    pub language: String,
    /// If there are more than one possibles languages, the first language
    /// (alphabetically) will be returned, and this field will be set to false.
    pub safe: bool,
}

impl From<GoGuess> for Guess {
    fn from(guess: GoGuess) -> Self {
        Self {
            language: guess.language.to_string(),
            safe: guess.safe == 1,
        }
    }
}
