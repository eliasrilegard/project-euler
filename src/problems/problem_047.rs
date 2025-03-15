/// Find the first four consecutive integers to have four distinct prime factors each. What is the
/// first of these numbers?
///
/// My solution for this problem requires setting an upper bound, requiring knowledge about the
/// problem which I consider meta.
///
/// If we express a number `N` as `N = p_1^n_1 * p_2^n_2 * ... * p_k^n_k`, the problem asks to find
/// the first of four numbers such that `k = 4` for all of them. We begin by having every prime
/// number `p` mark every multiple of itself `i * p` as having `p` as a factor. We then use a
/// sliding window approach to find the first instance of 4 consecutive numbers that all have 4
/// different prime factors.
pub fn solve() -> u32 {
  const TARGET: u32 = 4;

  let limit = 150_000; // Meta
  let mut factor_counts = vec![0; limit];

  // Sieve
  for i in 2..limit {
    if factor_counts[i] == 0 {
      // i is prime
      for multiple in (i..limit).step_by(i) {
        factor_counts[multiple] += 1;
      }
    }
  }

  // Sliding window
  factor_counts
    .windows(4)
    .enumerate()
    .find_map(|(i, window)| {
      window
        .iter()
        .all(|&count| count == TARGET)
        .then_some(i as u32)
    })
    .unwrap()
}
