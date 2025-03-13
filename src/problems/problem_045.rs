/// Find the next triangle number that is also pentagonal and hexagonal.
///
/// Any hexagonal number is also a triangular number, meaning we only have to check numbers being
/// both hexagonal and pentagonal. We can do this by generating hexagonal numbers and checking them
/// for whether they're pentagonal or not.
pub fn solve() -> u64 {
  let mut hexagonals = (144..).map(|n| n * (2 * n - 1));
  hexagonals.find(is_pentagonal).unwrap()
}

/// A number `n` is pentagonal if and only if `24n + 1` is a perfect square and
/// `sqrt(24n + 1) == 5 (mod 6)`.
fn is_pentagonal(n: &u64) -> bool {
  let square = 24 * n + 1;
  let root = (square as f64).sqrt() as u64;
  root * root == square && root % 6 == 5
}
