/// Find the sum of all numbers which are equal to the sum of the factorial of their digits.
/// Note: As 1! = 1 and 2! = 2 are not sums they are not included.
///
/// To find the limit, consider the number 99...9. At some point, `10^n >= n*9!`. The smallest `n`
/// satisfying this equation is `n = 7`. Once the limit is determined, it's a simple process of
/// filtering. The helpher function is optimized to not use string casting, which is slow.
pub fn solve() -> u32 {
  let factorials: Vec<u32> = (0..=9).map(|n| (1..=n).product()).collect();

  let limit = 7 * factorials[9];
  (10..=limit)
    .filter(|&n| n == digit_factorial_sum(n, &factorials))
    .sum()
}

fn digit_factorial_sum(mut n: u32, factorials: &[u32]) -> u32 {
  let mut sum = 0;
  while n > 0 {
    sum += factorials[(n % 10) as usize];
    n /= 10;
  }
  sum
}
