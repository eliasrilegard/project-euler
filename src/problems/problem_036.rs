use crate::math::misc::is_palindrome;

/// Find the sum of all numbers less than one million which are palindromic in base 10 and base 2.
/// (Please note that the palindromic number, in either base, may not include leading zeros.)
///
/// If a number is even, it will end in 0 in binary and will therefore not be palindromic in
/// binary. We can therefore skip out on checking all even numbers. If the number is determined to
/// not be palindromic in decimal, it will not be checked in binary.
pub fn solve() -> u32 {
  (1..1_000_000)
    .step_by(2)
    .filter(|&n| is_palindrome(n, 10) && is_palindrome(n, 2))
    .sum()
}
