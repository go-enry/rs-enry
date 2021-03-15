use core::convert::From;
use std::os::raw::c_uchar;

use crate::go::string::GoString;

#[repr(C)]
pub struct GoGuess {
    language: GoString,
    safe: c_uchar,
}


#[derive(Debug)]
pub struct Guess {
    language: String,
    safe: bool,
}

impl From<GoGuess> for Guess {
    fn from(guess: GoGuess) -> Self {
        Self {
            language: guess.language.to_string(),
            safe: guess.safe == 1,
        }
    }
}
