use crate::math::number_theory::gcd;

/// For which value of `p <= 1000` is the number of solutions for a right angled triangle with
/// integer side lengths maximized?
///
/// The hardest part of this solution is to generate all Pythagorean triplets. We can calculate
/// `c = p - (a + b)`. Additionally, `a < b < c` implies `a < p / 3`. The first equation now
/// implies that `b < p - (a + b)` which implies `b < (p - a) / 2`. These restrictions can be used
/// to narrow down the search space dramatically. However this solution uses number theory and
/// Euclid's formula to instead directly generate Pythagorean triplets.
pub fn solve() -> u32 {
  let mut max_solutions = 0;
  let mut best_p = 0;

  for p in 12..=1000 {
    let solutions = count_solutions(p);
    if solutions > max_solutions {
      max_solutions = solutions;
      best_p = p;
    }
  }

  best_p
}

/// Modify Euclid's formula to introduce a scaling factor `a = k * (m^2 - n^2)`, `b = k * 2mn`,
/// `c = k * (m^2 + n^2)`, where `m` and `n` are coprime and have opposite parity. Using
/// `p = a + b + c`, we get `p = k * (2m^2 + 2mn) = k * 2m(m + n)`. Iterating through possible
/// values of `m` and `n`, we can efficiently generate all possible triplets of interest.
fn count_solutions(target: u32) -> u32 {
  let mut count = 0;

  for m in 2..((target / 2) as f64).sqrt() as u32 {
    let parity = 1 + m % 2;

    for n in (parity..m).step_by(2).filter(|&n| gcd(m, n) == 1) {
      let p = 2 * m * (m + n);

      if p > target {
        break;
      }

      // Test for some k such that `k * p == target`
      if target % p == 0 {
        count += 1;
      }
    }
  }

  count
}
