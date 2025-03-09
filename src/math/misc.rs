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

/// Test whether an input is pandigital, i.e. contains (only) all digits between 1 and 9.
pub fn is_pandigital(input: &str) -> bool {
  if input.len() != 9 || input.contains("0") {
    return false;
  }

  let mut digits: Vec<char> = input.chars().collect();
  digits.sort_unstable();
  digits == ['1', '2', '3', '4', '5', '6', '7', '8', '9']
}

