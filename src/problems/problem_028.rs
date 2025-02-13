/// What is the sum of the numbers on the diagonals in a 1001 by 1001 number spiral like the
/// following 5 by 5 one?
/// 21 22 23 24 25
/// 20  7  8  9 10
/// 19  6  1  2 11
/// 18  5  4  3 12
/// 17 16 15 14 13
///
/// Observations:
/// - The top right corner is always a perfect square (assuming n is odd and only odd).
/// - The other three corners are evenly spaced from each other and the top right corner. The top
/// left corner can be computed as `n^2 - (n - 1)`, the bottom left as `n^2 - (n - 1) * 2`, etc.
/// - The sum of all four corners can therefore be expressed as `4n^2 - 6(n - 1)`.
///
/// Given the sum of all four corners in a layer, we can just sum them up across layers. The
/// formula doesn't hold for `n = 1`, so we can just compensate and add that at the end.
pub fn solve() -> u32 {
  let sum: u32 = (3..=1001).step_by(2).map(|n| 4 * n * n - 6 * (n - 1)).sum();

  1 + sum
}
