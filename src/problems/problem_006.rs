/// Find the difference between the sum of the squares of the first one hundred natural numbers and
/// the square of the sum.
///
/// This solution leverages closed-form expressions of different sums. The sum of the first `n`
/// natural numbers can be written as `n * (n + 1) / 2`. The sum of squares of the first n natural
/// numbers can be written as `n * (n + 1) * (2 * n + 1) / 6`.
pub fn solve() -> u32 {
  let n: u32 = 100;
  let sum_squared = (n * (n + 1) / 2).pow(2);
  let squares_sum = n * (n + 1) * (2 * n + 1) / 6;
  sum_squared - squares_sum
}
