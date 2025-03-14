use std::collections::{HashSet, VecDeque};

use crate::math::number_theory::is_prime;

/// Find the sum of the only eleven primes that are both truncatable from left to right and right
/// to left. Note: 2, 3, 5, 7 are not considered to be truncatable primes.
///
/// The primes which are both left to right and right to left truncatable can be computed as the
/// intersection of the left to right truncatable primes and the right to left truncatable primes.
/// Rather than going through all numbers and testing for the condition to hold, we can instead
/// build up the sets from the conditions.
pub fn solve() -> u32 {
  let mut right_truncatable = HashSet::new();
  let mut queue: VecDeque<u32> = VecDeque::from([2, 3, 5, 7]);
  while let Some(current) = queue.pop_front() {
    // Primes can only end in 1, 3, 7, or 9
    for y in [1, 3, 7, 9] {
      let candidate = current * 10 + y; // Add digit at the end
      if is_prime(candidate) {
        queue.push_back(candidate);
        right_truncatable.insert(candidate);
      }
    }
  }

  // Limit the next set to only have elements as large as the max of this one
  let maximum = right_truncatable.iter().max().unwrap_or(&0);

  let mut left_truncatable = HashSet::new();
  queue = VecDeque::from([2, 3, 5, 7]);
  while let Some(current) = queue.pop_front() {
    // Here we must test all digits
    for y in [1, 2, 3, 4, 5, 6, 7, 8, 9] {
      let candidate = y * 10_u32.pow(current.ilog10() + 1) + current; // Add digit in front
      if candidate <= *maximum && is_prime(candidate) {
        queue.push_back(candidate);
        left_truncatable.insert(candidate);
      }
    }
  }

  left_truncatable.intersection(&right_truncatable).sum()
}
