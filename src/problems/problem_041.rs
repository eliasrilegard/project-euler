use itertools::Itertools;

use crate::math::number_theory::is_prime;

/// What is the largest 1 to n-digit pandigital prime that exists?
///
/// A number which is 1 to 9 pandigital has a digit sum of 45, meaning it's a multiple of 3. To
/// find the answer we can generate all 1 to n pandigital numbers for `n <= 8` and iterate through
/// them in descending order, testing for primality. Once the first one is found, that's the
/// answer.
pub fn solve() -> u32 {
  for n in (1..=8).rev() {
    let digits: Vec<char> = (1..=n).map(|d| char::from_digit(d, 10).unwrap()).collect();
    let permutations = digits.iter().permutations(n as usize);

    // Iterate largest to smallest and return first found prime
    for permutation in permutations.sorted_unstable().rev() {
      let digits: String = permutation.iter().map(|&&c| c).collect();
      let num: u32 = digits.parse().unwrap();
      if is_prime(num as u64) {
        return num;
      }
    }
  }

  0 // None found
}
