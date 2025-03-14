/// What is the smallest odd composite that cannot be written as the sum of a prime and twice a
/// square?
///
/// We can test each odd composite number and return the first one we find. Use a dynamically
/// growing list of primes as a cache, both for determining number primality and for the Goldbach
/// test.
pub fn solve() -> u32 {
  let mut primes = vec![2, 3, 5, 7];
  for n in (9..).step_by(2) {
    if !is_composite(n, &mut primes) {
      continue;
    }
    if !goldbach_possible(n, &primes) {
      return n;
    }
  }
  0
}

/// Determine whether a number can be written on the form `n = p + 2s^2` for some prime `p` and
/// square number `s`.
fn goldbach_possible(n: u32, primes: &[u32]) -> bool {
  for p in primes.iter().take_while(|&&p| p < n) {
    let two_s2 = n - p;
    if two_s2 % 2 == 0 {
      let s2 = two_s2 / 2;
      let s = (s2 as f64).sqrt() as u32;

      // Is s^2 a perfect square?
      if s * s == s2 {
        return true;
      }
    }
  }

  false
}

/// Determine whether a number is composite, using a cache of primes that dynamically grows as we
/// test bigger numbers.
fn is_composite(n: u32, primes: &mut Vec<u32>) -> bool {
  if primes.contains(&n) {
    return false;
  }

  let limit = (n as f64).sqrt() as u32;
  if primes
    .iter()
    .take_while(|&&p| p <= limit)
    .all(|&p| n % p != 0)
  {
    primes.push(n);
    return false;
  }

  true
}
