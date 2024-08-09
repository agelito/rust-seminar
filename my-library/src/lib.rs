//! My first Rust Library
//!
//! This is just an example library to demonstrate how libraries is written and interacted with in
//! the rust programming language.

/// Add two numbers and return the sum
///
/// The left parameter is added to the right parameter.
///
/// # Panics
/// The function will panic if result doesn't fit within `usize`.
///
/// # Examples
///
/// ```
/// let result = my_library::add(2, 5);
/// assert_eq!(result, 7);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// Multiply two numbers and return the result
///
/// The left parameter is multiplied with the right parameter.
///
/// # Panics
/// The function will panic if result doesn't fit within `usize`.
///
/// # Examples
///
/// ```
/// let result = my_library::mul(5, 5);
/// assert_eq!(result, 25);
/// ```
pub fn mul(left: usize, right: usize) -> usize {
    left * right
}


/// Divide two numbers and return the result
///
/// The left parameter is divided with the right parameter.
///
/// # Panics
/// The function will panic if the right argument is 0.
///
/// # Examples
///
/// ```
/// let result = my_library::div(10, 2);
/// assert_eq!(result, 5);
/// ```
pub fn div(left: usize, right: usize) -> usize {
    left / right
}

/// Subtract two numbers and return the result
///
/// The right parameter is subtracted from the left parameter.
///
/// # Panics
/// The function will panic if the right arguments is bigger than the left argument.
///
/// # Examples
///
/// ```
/// let result = my_library::sub(16, 5);
/// assert_eq(result, 11);
/// ```
pub fn sub(left: usize, right: usize) -> usize {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_returns_sum() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn mul_multiplies() {
        let result = mul(5, 5);
        assert_eq!(result, 25);
    }

    #[test]
    fn div_divides() {
        let result = div(10, 2);
        assert_eq!(result, 5);
    }

    #[test]
    fn sub_subtracts() {
        let result = sub(16, 5);
        assert_eq!(result, 11);
    }
}
