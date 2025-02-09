/// n choose k
pub fn count_combinations(n: u64, k: u64) -> u64 {
  if k > n {
    0
  } else {
    (1..=k.min(n - k)).fold(1, |acc, val| acc * (n - val + 1) / val)
  }
}
