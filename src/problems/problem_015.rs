use crate::math::combinatorics::count_combinations;

/// Starting in the top left corner of a 20x20 grid and only being able to move to the right and
/// down, how many routes are there to the bottom corner?
///
/// Note that we need to walk a total of 40 steps (20 across and 20 down), and what makes the path
/// unique is where the steps down are in relation to the steps across. In essence, we need to
/// select 20 of the 40 steps to be steps down, leaving the rest to be steps across. How many ways
/// are there to select 20 steps from 40 possible? Answer: 40 choose 20
pub fn solve() -> u64 {
  count_combinations(40, 20)
}
