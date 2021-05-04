/// This function divides two numbers.
///
/// # Example #1: 10 / 2 == 5
///
/// ```
/// let result = doctest001::div(10, 2);  // TODO: finish this test!
/// assert_eq!(result, 5);
/// ```
///
/// # Example #2: 6 / 2 = 3
///
/// ```
/// let result = doctest001::div(6, 2);
/// assert_eq!(result, 3);
/// ```
///
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// doctest001::div(1, 0); 
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }
    a / b
}

/// This function subtracts two numbers.
///
/// # Example #1: 9 - 2 == 7
///
/// ```
/// let result = doctest001::sub(9, 2);
/// assert_eq!(result, 7);
/// ```
///
/// # Example #2: 6 - 9 == -3
///
/// ```
/// let result = doctest001::sub(6, 9);
/// assert_eq!(result, -3);
/// ```
pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}