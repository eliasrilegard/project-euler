use crate::math::misc::is_palindrome;

/// Find the largest palindrome made from the product of two 3-digit numbers.
///
/// We are looking for the largest palindrome, therefore it makes sense to iterate backwards, down
/// from 999. This way we are hitting the larger products earlier, allowing us to break out of the
/// loop more often. The inner loop also avoids re-computing the same number, as a * b == b * a.
pub fn solve() -> u32 {
  let mut maximum = 0;

  for i in (101..1000).rev() {
    for j in (i..1000).rev() {
      let product = i * j;
      if product <= maximum {
        break;
      }
      if is_palindrome(product) {
        maximum = product;
      }
    }
  }

  maximum
}
