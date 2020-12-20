#![doc(html_logo_url = "https://d30y9cdsu7xlg0.cloudfront.net/png/411962-200.png")]
#![doc(html_playground_url = "https://play.rust-lang.org/")]

//! This is a logic gates simulation crate built to demonstrate writing unit tests
//! and integration tests [![Build Status](https://www.travis-ci.com/Soupayan/logic_gates.svg?branch=master)](https://www.travis-ci.com/Soupayan/logic_gates)
//! ```
//! use logic_gates::and;
//! assert_eq!(1, and(1,1))
//! ```
/// This is a document for and function
pub fn and(a: u8, b: u8) -> u8 {
    match (a, b) {
        (1, 1) => 1,
        _ => 0,
    }
}

/// This is a document for xor function
pub fn xor(a: u8, b: u8) -> u8 {
    match (a, b) {
        (1, 1) | (0, 0) => 0,
        _ => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::{and, xor};
    #[test]
    fn test_and() {
        assert_eq!(1, and(1, 1));
        assert_eq!(0, and(0, 0));
        assert_eq!(0, and(0, 1));
        assert_eq!(0, and(1, 0));
    }

    #[test]
    fn test_xor() {
        assert_eq!(0, xor(1, 1));
        assert_eq!(0, xor(0, 0));
        assert_eq!(1, xor(0, 1));
        assert_eq!(1, xor(1, 0));
    }
}
