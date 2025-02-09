use num_bigint::BigUint;

/// What is the index of the first term in the Fibonacci sequence to contain 1000 digits?
///
/// If a number contains 1000 digits, it is by definition >= 10^999. Using arbitrary precision
/// arithmetic we can quickly compute the solution.
pub fn solve() -> u32 {
  let mut a = BigUint::ZERO;
  let mut b = BigUint::from(1u32);
  let mut index = 1;

  let limit = BigUint::from(10u32).pow(999);
  while b < limit {
    let temp = b.clone();
    b += a;
    a = temp;
    index += 1;
  }

  index
}
