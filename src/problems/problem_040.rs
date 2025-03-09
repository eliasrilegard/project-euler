/// Find the value of the expression `d_1 * d_10 * ... * d_1000000` where `d_i` is the i-th decimal
/// in Champernowne's constant.
///
/// Rather than constructing the constant and indexing into it, we compute each digit of interest
/// directly.
pub fn solve() -> u32 {
  let targets = [1, 10, 100, 1000, 10_000, 100_000, 1_000_000];
  targets.iter().map(|&i| nth_champernowne(i)).product()
}

/// Determine which number contains the target digit and extract it directly, rather than
/// constructing and storing the entire constant.
fn nth_champernowne(target: usize) -> u32 {
  let mut index = 0;
  let mut num_digits = 1;
  let mut count = 9;
  let mut range_start = 1;

  while index + num_digits * count < target {
    // Move past this range
    index += num_digits * count;

    // Properties of next range
    num_digits += 1;
    count *= 10;
    range_start *= 10;
  }

  // Determine the exact number containing the target index
  let offset = target - index - 1;
  let number = range_start + offset / num_digits;

  // Extract correct digit
  let digit = number.to_string().chars().nth(offset % num_digits).unwrap();
  digit.to_digit(10).unwrap()
}
