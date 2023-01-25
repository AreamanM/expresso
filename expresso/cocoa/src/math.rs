//! Helper functions which handle some of the arithmetic that expresso supports.

/// Calculate the factorial of n where n is an integer that is greater
/// than or equal to 0.
///
/// This implementation does not use the gamma function, hence factorials
/// of negative values or non integers cannot be computed and will result
/// in an infinite loop.
///
/// # Arguments
///
/// * `n` - An integer >= 0.
///
/// # Examples
/// ```
/// use cocoa::math::ufactorial;
///
/// assert_eq!(ufactorial(0), 1);
/// assert_eq!(ufactorial(5), 120);
/// assert_eq!(ufactorial(7), 5040);
/// ```
pub fn ufactorial(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }

    n * ufactorial(n - 1)
}
