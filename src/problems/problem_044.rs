/// Find the pair of pentagonal numbers, `P_j` and `P_k`, for which their sum and difference are
/// pentagonal and `D = |P_j - P_k|` is minimised; what is the value of `D`?
///
/// From the formula generating the pentagonal numbers, `P_n = n * (3n - 1) / 2`, we note that the
/// difference between two pentagonal numbers increases as `n` increases. Therefore, we're looking
/// to find the smallest pair of pentagonal numbers with the desired property. We generate all
/// pentagonal numbers as we need them (and store them for future use), then return as soon as the
/// first pair is found.
pub fn solve() -> u32 {
  let mut pentagonals = vec![];

  for j in 1.. {
    let pj = pentagonal(j);
    pentagonals.push(pj);

    for k in (0..j as usize).rev() {
      let pk = pentagonals[k];
      let sum = pj + pk;
      let diff = pj - pk;

      if is_pentagonal(sum) && is_pentagonal(diff) {
        return diff; // First hit is the smallest
      }
    }
  }

  0 // Never reached
}

/// Returns the nth pentagonal number.
fn pentagonal(n: u32) -> u32 {
  n * (3 * n - 1) / 2
}

/// A number `n` is pentagonal if and only if `24n + 1` is a perfect square and
/// `sqrt(24n + 1) == 5 (mod 6)`.
fn is_pentagonal(n: u32) -> bool {
  let square = 24 * n + 1;
  let root = (square as f64).sqrt() as u32;
  root * root == square && root % 6 == 5
}
