/// Find the sum of all the positive integers which cannot be written as the sum of two abundant
/// numbers.
///
/// By first computing all abundant numbers less than `LIMIT`, we can then simply test combining
/// every one and noting which numbers *can* be written as the sum of two abundant numbers. After
/// that we filter those numbers out and take the remaining sum.
pub fn solve() -> u32 {
  const LIMIT: usize = 28_123;

  let sums_of_divisors = divisors_sums(LIMIT);
  let abundant_numbers: Vec<usize> = (1..=LIMIT)
    .filter(|&n| sums_of_divisors[n] > n)
    .collect();

  // Mark all numbers which can be written as the sum of two abundant numbers
  let mut is_sum_of_abundants = vec![false; LIMIT + 1];
  for (i, &a) in abundant_numbers.iter().enumerate() {
    for &b in &abundant_numbers[i..] { // Avoid duplicate checks by starting b at a
      let sum = a + b;
      if sum > LIMIT { break; }
      is_sum_of_abundants[sum] = true;
    }
  }

  (1..LIMIT)
    .filter(|&n| !is_sum_of_abundants[n])
    .sum::<usize>() as u32
}

// Compute the sums of divisors for every number in a sieve-like fashion.
fn divisors_sums(limit: usize) -> Vec<usize> {
  let mut sums_of_divisors = vec![0; limit + 1];

  for i in 1..=limit / 2 {
    for j in (i * 2..=limit).step_by(i) {
      sums_of_divisors[j] += i;
    }
  }

  sums_of_divisors
}
