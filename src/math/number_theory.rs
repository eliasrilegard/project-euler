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
