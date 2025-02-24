use crate::math::number_theory::gcd;

/// Find the denominator of the reduced product of the four fractions that can be incorrectly
/// simplified.
///
/// The simplest way of keeping track of the found fractions is storing a tally of the products.
pub fn solve() -> u32 {
  let mut num_product = 1;
  let mut den_product = 1;

  for num in 10..100 {
    for den in (num + 1)..100 {
      if is_curious(num, den) {
        num_product *= num;
        den_product *= den;
      }
    }
  }

  den_product / gcd(num_product, den_product)
}

/// A "curious" fraction can have the same digit cancelled from the numerator and denominator
/// without affecting the value of the fraction.
fn is_curious(numerator: u32, denominator: u32) -> bool {
  let num_digits = (numerator / 10, numerator % 10); // (tens, ones)
  let den_digits = (denominator / 10, denominator % 10);

  // Exclude trivial cases
  if num_digits.1 == 0 && den_digits.1 == 0 {
    return false;
  }

  let original_fraction = (numerator as f64) / (denominator as f64);

  // Check all valid digit cancellations
  if num_digits.0 == den_digits.0 && den_digits.1 != 0 {
    return (num_digits.1 as f64) / (den_digits.1 as f64) == original_fraction;
  }
  if num_digits.0 == den_digits.1 && den_digits.0 != 0 {
    return (num_digits.1 as f64) / (den_digits.0 as f64) == original_fraction;
  }
  if num_digits.1 == den_digits.0 && den_digits.1 != 0 {
    return (num_digits.0 as f64) / (den_digits.1 as f64) == original_fraction;
  }
  if num_digits.1 == den_digits.1 && den_digits.0 != 0 {
    return (num_digits.0 as f64) / (den_digits.0 as f64) == original_fraction;
  }

  false
}
