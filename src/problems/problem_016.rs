use num_bigint::BigInt;

/// What is the sum of the digits of the number 2^1000?
///
/// BigInt allows easy computation of very big numbers.
pub fn solve() -> u32 {
  BigInt::from(2)
    .pow(1000)
    .to_string()
    .chars()
    .map(|c| c.to_digit(10).unwrap())
    .sum()
}
