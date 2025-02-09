/// By considering the terms in the Fibonacci sequence whose values do not exceed four million,
/// find the sum of the even-valued terms.
///
/// This solution simply generates the Fibonacci numbers and adds together only the even ones.
pub fn solve() -> u32 {
  const LIMIT: u32 = 4_000_000;
  let mut sum = 0;

  let (mut a, mut b) = (0, 1);
  while b <= LIMIT {
    if b % 2 == 0 {
      sum += b;
    }
    (a, b) = (b, a + b);
  }

  sum
}
