/// Find the value of `d < 1000` for which `1 / d` contains the longest recurring cycle in its
/// decimal fraction part.
///
/// Solve this problem by simulating long division. At every step, keep track of seen remainders,
/// as soon as a remainder repeats we know we have a cycle.
pub fn solve() -> u32 {
  (2..1000)
    .max_by_key(|&d| cycle_length(d))
    .unwrap()
}

/// Long division simulation computing the length of the cycle corresponding to the reciprocal of a
/// number.
fn cycle_length(n: u32) -> u32 {
  let mut remainders = vec![0; n as usize];
  let mut value = 1; // Numerator
  let mut position = 0;

  while value != 0 {
    if remainders[value] != 0 {
      // Cycle might not begin at position 0. See 1/6
      return position - remainders[value];
    }

    remainders[value] = position;
    value = (value * 10) % n as usize;
    position += 1;
  }

  0 // Algorithm terminates, no cycle
}
