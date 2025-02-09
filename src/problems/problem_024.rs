/// What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
///
/// For the first digit, there are `n` possibilities. Sorted lexiographically, each possible digit
/// accounts for `(n - 1)!` combinations. We want to select the digit that gets us as close as
/// possible to the millionth combination without overshooting. Once that digit is found, we
/// consume it and repeat the process starting from how close the previous iteration got us. Now,
/// there are `n - 1` possibilities for the next digit with each remaining digit accounting for
/// `(n - 2)!` combinations. Repeat the process until all digits are found.
pub fn solve() -> String {
  let mut digits: Vec<u32> = (0..10).collect();
  let mut target = 1_000_000 - 1; // Zero-indexed
  let mut result = String::new();
  
  // Precompute the factorials of 0 to 9
  let mut factorials = vec![1; 10];
  for i in 1..10 {
    factorials[i] = factorials[i - 1] * i as u32;
  }

  // Extract index by dividing by `(possible numbers)!`, find corresponding digit
  for i in (1..=9).rev() {
    let index = target / factorials[i];
    let digit = digits.remove(index as usize);
    result.push((b'0' + digit as u8) as char);
    target %= factorials[i];
  }

  result.push((b'0' + digits[0] as u8) as char); // Last remaining digit is the final one
  result
}
