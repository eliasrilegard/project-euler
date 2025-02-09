use crate::math::number_theory::prime_factors;

/// What is the largest prime factor of the number 600851475143?
///
/// This solution has `prime_factors` return all prime factors of the number in ascending order.
/// Therefore we can simply take the last in that array as our answer.
pub fn solve() -> u64 {
  let factors = prime_factors(600_851_475_143);
  factors.last().unwrap().to_owned()
}
