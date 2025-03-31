use std::collections::HashSet;

use crate::math::number_theory::sieve_of_eratosthenes;

/// Find the smallest prime which, by replacing part of the number (not necessarily adjacent
/// digits) with the same digit, is part of an eight prime value family.
///
/// First non 5% difficulty problem! Here we generate all possible "masks" for a given number and
/// for each mask try replace with every possible number. Keep track of how many primes we
/// encounter for each mask and return as soon as we find the first 8-prime family.
pub fn solve() -> u32 {
  let limit: u32 = 1_000_000;
  let primes = sieve_of_eratosthenes(limit);
  let prime_set: HashSet<u32> = primes.iter().cloned().collect();

  // Precompute collection of every possible mask for every possible length, each represented as an
  // array of booleans (should replace or not)
  let masks: Vec<Vec<Vec<bool>>> = (0..limit.to_string().len()).map(generate_mask).collect();

  for &prime in &primes {
    let digits = prime.to_string();
    let len = digits.len();
    let max_value = 10_u32.pow(len as u32 - 1);

    for mask in &masks[len] {
      let mut prime_family = vec![];

      for replacement in 0..=9 {
        let candidate = apply_mask(&digits, mask, replacement);
        if candidate >= max_value && prime_set.contains(&candidate) {
          prime_family.push(candidate);
        }
      }

      // Break immediately, will be smallest value
      if prime_family.len() == 8 {
        return *prime_family.iter().min().unwrap();
      }
    }
  }

  0 // Unreachable given a sufficient limit
}

/// Generate all possible replacement masks (e.g. [false, false, true, true, false] for 56**3).
fn generate_mask(len: usize) -> Vec<Vec<bool>> {
  let mut masks = vec![];

  // Avoid modifying last digit (`mask[len - 1]` is always false)
  for bits in 1..(1 << (len.max(1) - 1)) {
    let mask: Vec<bool> = (0..len).map(|i| (bits >> i) & 1 == 1).collect();

    // Shortcut: Only replace 3n digits. This preserves the divisibility by 3 (sum of digits),
    // meaning the masked number will be divisible by 3 if and only if the original number is.
    if mask.iter().filter(|&&b| b).count() % 3 == 0 {
      masks.push(mask);
    }
  }

  masks
}

/// Replace digits in a number (given as a string digit representation) given a mask and a value
/// for all replacements.
fn apply_mask(digits: &str, do_replace: &[bool], replacement: u32) -> u32 {
  let new_digits: String = digits
    .chars()
    .enumerate()
    .map(|(i, c)| {
      if do_replace[i] {
        char::from_digit(replacement, 10).unwrap()
      } else {
        c
      }
    })
    .collect();

  new_digits.parse().unwrap()
}
