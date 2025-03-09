use std::collections::HashSet;

use crate::math::misc::is_pandigital;

/// Find the sum of all products whose multiplicand/multiplier/product identity can be written as a
/// 1 through 9 pandigital. Hint: Some products can be obtained in more than one way so be sure to
/// only include it once in your sum.
///
/// There core of the algorithm is a brute-force search through all possible procuts `a * b = c`
/// such that `a`, `b`, and `c` together are pandigital. There are a couple of optimizations to be
/// had, however. We can first note that the total length of `a`, `b`, and `c` must be 9, which
/// places the restriction on `a` and `b` in that they can together at most have 5 digits. This
/// allows us to reduce the search space down to two cases: one case as a 4-digit times a 1-digit
/// and the other as a 3-digit times a 2-digit. Furthermore, there is no need to check `1234 * 5`
/// if we've already checked `5 * 1234`, meaning we can prune half of the remaining search space.
/// Finally, to avoid overcounting certain products, we can use a set storing the final product.
pub fn solve() -> u32 {
  let mut products = HashSet::new();

  // Case 1: 1-digit * 4-digit = 4-digit
  for a in 1..10 {
    for b in 1000..10_000 {
      let c = a * b;
      if c < 10_000 && is_pandigital(&format!("{a}{b}{c}")) {
        products.insert(c);
      }
    }
  }

  // Case 2: 2-digit * 3-digit = 4-digit
  for a in 10..100 {
    for b in 100..1000 {
      let c = a * b;
      if c < 10_000 && is_pandigital(&format!("{a}{b}{c}")) {
        products.insert(c);
      }
    }
  }

  products.iter().sum()
}
