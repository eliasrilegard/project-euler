/// Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.
///
/// The sum of the digits to the fifth power of a number is `9^5 * k`, where `k` is the number of
/// digits in the number. At some point the base-10 notation will exceed the maximum sum formed in
/// this way. We can express this as `10^k > 9^5 * k`, which has `k = 6` as the smallest solution.
/// The corresponding limit is then `9^5 * 6 = 354_294`.
pub fn solve() -> u32 {
  let limit = 354_294;

  (10..=limit)
    .filter(|&n| {
      let sum_of_5th_powers: u32 = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(5))
        .sum();
      sum_of_5th_powers == n
    })
    .sum()
}
