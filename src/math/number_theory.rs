/// Computes the greatest common divisor between a and b.
/// The greatest common divisor is the largest positive integer dividing both and and b.
pub fn gcd(a: u32, b: u32) -> u32 {
  if b == 0 {
    a
  } else {
    gcd(b, a % b)
  }
}

/// Computes the least common multiple of a and b.
/// The least common multiple is the smallest integer that is a multiple of both a and b.
pub fn lcm(a: u32, b: u32) -> u32 {
  a * (b / gcd(a, b)) // Prevent overflow by dividing first
}

/// Simple function to determine whether a number is prime or not.
pub fn is_prime(n: u32) -> bool {
  if n < 2 {
    return false;
  }
  if n == 2 || n == 3 {
    return true;
  }
  if n % 2 == 0 || n % 3 == 0 {
    return false;
  }

  let mut i = 5;
  while i * i <= n {
    if n % i == 0 || n % (i + 2) == 0 {
      return false;
    }
    i += 6;
  }

  true
}

/// The Sieve of Eratosthenes efficiently generates all primes up to a given upper bound.
/// Time complexity `O(n log log n)`.
pub fn sieve_of_eratosthenes(max: u32) -> Vec<u32> {
  let limit = max as usize;
  let mut primes = vec![true; limit + 1];
  let mut result = Vec::new();

  let sqrt_limit = (limit as f64).sqrt() as usize;
  for n in 2..=sqrt_limit {
    if primes[n] {
      result.push(n as u32);
      let mut multiple = n * n;
      while multiple <= limit {
        primes[multiple] = false;
        multiple += n;
      }
    }
  }

  // Collect all remaining primes beyond
  #[allow(clippy::needless_range_loop)] // Speed
  for n in sqrt_limit + 1..limit {
    if primes[n] {
      result.push(n as u32);
    }
  }

  result
}

/// Compute all prime factors of a number. The resulting list is ordered in ascending order.
pub fn prime_factors(n: u64) -> Vec<u64> {
  let mut factors = Vec::new();
  let mut num = n;

  // Handle divisibility by 2
  while num % 2 == 0 {
    factors.push(2);
    num /= 2;
  }

  // Check odd factors starting at 3
  let mut i = 3;
  while i * i <= num {
    while num % i == 0 {
      factors.push(i);
      num /= i;
    }
    i += 2;
  }

  // If the number is greater than 2, it's prime
  if num > 2 {
    factors.push(num);
  }

  factors
}
