// documentation comments can be useful for describing crates and modules

//! # Create Documentation
//! 
//! 'create_documentation' is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
/// 
/// # Examples
/// ```
/// let arg = 5;
/// let answer = create_documentation::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
pub fn add_one(value: u64) -> u64 {
    value + 1
}

// run:
// - cargo doc
// - cargo doc --open
// the code examples in the documentation will be run as part of the tests
#[cfg(test)]
mod tests {
    use super::*;
}
