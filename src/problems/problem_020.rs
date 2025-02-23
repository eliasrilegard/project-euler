use num_bigint::BigUint;

/// Find the sum of the digits in the number `100!`.
///
/// `num-bigint` allows for a very neat solution.
pub fn solve() -> u32 {
  let n: u32 = 100;

  let factorial: BigUint = (1..=n).map(BigUint::from).product();

  factorial
    .to_string()
    .chars()
    .map(|c| c.to_digit(10).unwrap())
    .sum()
}
