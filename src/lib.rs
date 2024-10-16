//! ## Mathcalculate, A math library!
//! This is my first Rust library, too.

/// Adds 'first' to 'second'
/// 
/// Example:
/// ```rust
/// use mathcalculate::add;
/// 
/// fn main() {
///     println!("{}", add(5, 8))
/// }
/// ```
pub fn add(first: usize, second: usize) -> usize {
    first + second
}

/// Removes 'first' from 'second'
/// 
/// Example:
/// ```rust
/// use mathcalculate::remove;
/// 
/// fn main() {
///     println!("{}", remove(5, 8))
/// }
/// ```
pub fn remove(first: usize, second: usize) -> usize {
    first - second
}

/// Multiplies 'first' by 'second'
/// 
/// Example:
/// ```rust
/// use mathcalculate::multiply;
/// 
/// fn main() {
///     println!("{}", multiply(5, 8))
/// }
/// ```
pub fn multiply(first: usize, second: usize) -> usize {
    first * second
}

/// Divides 'first' by 'second'
/// 
/// Example:
/// ```rust
/// use mathcalculate::divide;
/// 
/// fn main() {
///     println!("{}", divide(5, 8))
/// }
/// ```
pub fn divide(first: usize, second: usize) -> usize {
    first / second
}

