/// There exists exactly one Pythagorean triplet for which `a + b + c = 1000`. Find the product
/// `a * b * c`.
///
/// The scope of this problem is small enough to have a brute-force search be an option. Iterating
/// over possible values of `a` and `b`, we check if the triplet has the desired sum, at which
/// point we return the computed product. If the search concludes without a hit, then no
/// Pythagorean triplet has the desired sum.
pub fn solve() -> u32 {
  find_pythagorean_product(1000).unwrap()
}

/// Brute force search for a pythagorean triplet with a desired total sum. Optimized by bounding
/// `b` by `a`, which in turn puts a bound on `c`.
fn find_pythagorean_product(target_sum: u32) -> Option<u32> {
  for a in 1..target_sum {
    for b in a..target_sum - a {
      let c = target_sum - a - b;
      if a * a + b * b == c * c {
        return Some(a * b * c);
      }
    }
  }
  None
}
