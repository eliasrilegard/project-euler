use std::ops::{Add, Div, Mul, Rem};

/// Check whether a number is palindrome, i.e. the same when written backwards as forwards.
pub fn is_palindrome<T>(mut n: T, base: T) -> bool
where
  T: Copy + PartialEq + From<u8> + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Rem<Output = T>,
{
  let original = n;
  let mut reversed = T::from(0);

  while n != T::from(0) {
    reversed = reversed * base + (n % base);
    n = n / base;
  }

  reversed == original
}
