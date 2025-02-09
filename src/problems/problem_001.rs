/// Find the sum of all multiples of 3 or 5 below 1000.
///
/// To solve this, we can take the range of numbers up to 1000, filter out any numbers which isn't
/// a multiple of 3 or 5, and then summing them up.
pub fn solve() -> u32 {
  (1..1000).filter(|&x| x % 3 == 0 || x % 5 == 0).sum()
}
