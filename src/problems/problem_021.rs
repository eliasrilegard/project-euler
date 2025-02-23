/// Evaluate the sum of all the amicable numbers under `10_000`.
///
/// To solve this, we check every number and test for whether the sum of its divisors, `sum`, has
/// the property that the sum of the divisors of `sum` is equal to the original number.
pub fn solve() -> u32 {
  const LIMIT: u32 = 10_000;
  let divisor_sums: Vec<u32> = (0..LIMIT).map(sum_of_divisors).collect();

  (1..LIMIT)
    .filter(|&n| {
      let sum = divisor_sums[n as usize];
      sum < LIMIT && sum != n && divisor_sums[sum as usize] == n
    })
    .sum()
}

/// Efficiently (O(√n)) compute the sum of all proper divisors of a number.
/// This function is a mirror of `number_theory::proper_divisors` but does not store the divisors
/// it computes for the sake of efficiency.
fn sum_of_divisors(n: u32) -> u32 {
  if n < 2 {
    return n;
  }

  let mut sum = 1; // 1 is always a divisor
  let sqrt = (n as f64).sqrt() as u32;

  for i in 2..=sqrt {
    if n % i == 0 {
      sum += i;
      let pair = n / i;
      if pair != i {
        sum += pair;
      }
    }
  }

  sum
}
