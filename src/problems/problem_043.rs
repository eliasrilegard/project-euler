/// Find the sum of all 0 to 9 pandigital numbers with the property that has the substring
/// divisibility property.
///
/// The simple solution to this problem is to generate all possible permutations (e.g. using
/// Itertools), filter out the numbers which have the divisibility property and then sum them up.
/// We can improve this solution by generating the numbers in-place over a loop and summing them
/// using a running tally.
pub fn solve() -> u64 {
  let mut digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
  let mut sum = 0;

  while next_permutation(&mut digits) {
    if has_divisibility_property(&digits) {
      sum += to_number(&digits)
    }
  }

  sum
}

/// Generates the next lexicographic permutation (modifies `arr` in-place).
/// Returns `false` if it was the last permutation.
fn next_permutation(arr: &mut [u32]) -> bool {
  let n = arr.len();
  if n < 2 {
    return false;
  }

  // 1. Find the rightmost element smaller than its successor
  let mut i = n - 2;
  while i > 0 && arr[i] >= arr[i + 1] {
    i -= 1;
  }

  if i == 0 && arr[0] >= arr[1] {
    return false; // No more permutations
  }

  // 2. Find the rightmost element larger than arr[i]
  let mut j = n - 1;
  while arr[j] <= arr[i] {
    j -= 1;
  }

  // 3. Swap elements at position `i` and `j`
  arr.swap(i, j);

  // 4. Reverse sequence beyond position `i`
  arr[i + 1..].reverse();

  true
}

/// Checks if a 0-9 pandigital number (given as a slice of digits) satisfies the divisibility
/// property.
fn has_divisibility_property(digits: &[u32]) -> bool {
  const PRIMES: [u32; 7] = [2, 3, 5, 7, 11, 13, 17];

  // Can be written directly with a `.all()` call on the enumerated PRIMEs, but I've left it like
  // this because I find this more understandable
  for (i, &prime) in PRIMES.iter().enumerate() {
    if (digits[i + 1] * 100 + digits[i + 2] * 10 + digits[i + 3]) % prime != 0 {
      return false;
    }
  }

  true
}

/// Convert a slice of digits to a number.
fn to_number(digits: &[u32]) -> u64 {
  digits.iter().fold(0, |acc, &d| acc * 10 + d as u64)
}

// // Simpler approach, but ~32x slower
// use itertools::Itertools as _;
// pub fn solve() -> u64 {
//   let permutations = (0..=9).permutations(10);
//
//   permutations
//     .filter_map(|perm| {
//       if has_divisibility_property(&perm) {
//         Some(to_number(&perm))
//       } else {
//         None
//       }
//     })
//     .sum()
// }
