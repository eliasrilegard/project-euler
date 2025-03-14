use std::collections::HashSet;

use crate::math::number_theory::sieve_of_eratosthenes;

/// How many circular primes are there below one million?
///
/// Any prime containing any even digit or 5 can immediately be discarded, since they will at some
/// point not be a prime. Cycling digits is faster numerically than with using strings.
pub fn solve() -> usize {
  let limit = 1_000_000;
  let primes: HashSet<u32> = sieve_of_eratosthenes(limit).into_iter().collect();

  primes
    .iter()
    .filter(|&&p| is_circular_prime(p, &primes))
    .count()
}

fn is_circular_prime(n: u32, primes: &HashSet<u32>) -> bool {
  let num_digits = n.ilog10() + 1;

  if num_digits > 1 && has_even_or_five(n) {
    return false;
  }

  let mut digits = n;
  for _ in 0..num_digits {
    if !primes.contains(&digits) {
      return false;
    }
    digits = cycle_digits(digits, num_digits);
  }

  true
}

fn cycle_digits(number: u32, length: u32) -> u32 {
  let last_digit = number % 10;
  let rest = number / 10;
  last_digit * 10_u32.pow(length - 1) + rest
}

fn has_even_or_five(mut number: u32) -> bool {
  while number > 0 {
    let d = number % 10;
    if d % 2 == 0 || d == 5 {
      return true;
    }
    number /= 10;
  }

  false
}
