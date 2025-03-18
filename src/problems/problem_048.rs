/// Find the last ten digits of the series `1^1 + 2^2 + ... + 1000^1000`.
///
/// While it's possible to use BigInts here, it is slow. Staying with primitive types instead is
/// about ~10x faster. To do that, we use (fast) modular exponentiation to stay within bounds, but
/// as `(10^10)^2 > u64::MAX` we are forced to step up.
pub fn solve() -> u128 {
  const MOD: u128 = 10_000_000_000;

  let sum: u128 = (1..=1000).map(|n| mod_pow(n, n, MOD)).sum();

  sum % MOD
}

/// Fast modular exponentiation
fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
  let mut result = 1;
  base %= modulus;

  while exp > 0 {
    if exp % 2 == 1 {
      result = (result * base) % modulus;
    }

    base = (base * base) % modulus;
    exp /= 2;
  }

  result
}
