use crate::math::misc::is_pandigital;

/// What is the largest 1 to 9 pandigital 9-digit number that can be formed as the concatenated
/// product of an integer with (1, 2, ..., n) where n > 1?
///
/// If a number `n` has 4 digits, we require `2n` to have 5 digits for the concatenated number to
/// have 9 digits and thus a chance at being pandigital. This means we can require `n >= 5000`.
/// Likewise, if `n` has 2 digits, we require `4n` (and only `4n`) to have 3 digits for the
/// concatenated number to have 9 digits. This means we can here require `n >= 25` but also
/// `n <= 33`.
pub fn solve() -> u32 {
  let mut largest = 0;

  for x in 5..10 {
    check_multiples(x, 5, &mut largest);
  }
  for x in 25..=33 {
    check_multiples(x, 4, &mut largest);
  }
  for x in 100..=333 {
    check_multiples(x, 3, &mut largest);
  }
  for x in 5000..10_000 {
    check_multiples(x, 2, &mut largest);
  }

  largest
}

/// Test whether a number multiplied with `(1, 2, ..., n)` when concatenated is pandigital.
fn check_multiples(number: u32, n: u32, largest: &mut u32) {
  let concatenated: String = (1..=n).map(|i| (number * i).to_string()).collect();

  if is_pandigital(&concatenated) {
    let parsed: u32 = concatenated.parse().unwrap();
    *largest = (*largest).max(parsed);
  }
}
