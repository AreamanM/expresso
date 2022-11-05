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
/// assert_eq!(ufactorial(0_u64), 1_u64);
/// assert_eq!(ufactorial(5_u64), 120_u64);
/// assert_eq!(ufactorial(7_u64), 5040_u64);
/// ```
pub fn ufactorial(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }

    n * ufactorial(n - 1)
}
