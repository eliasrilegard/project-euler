use std::fs;

/// What is the total of all the name scores in the file `resources/0022_names.txt`?
///
/// Remember that arrays are zero-indexed!
pub fn solve() -> u32 {
  let input = fs::read_to_string("resources/0022_names.txt").unwrap_or_default();

  let mut names: Vec<&str> = input
    .split(",")
    .map(|name| name.trim_matches('"'))
    .collect();
  names.sort_unstable();

  names
    .iter()
    .enumerate()
    .map(|(i, name)| (i as u32 + 1) * name_score(name))
    .sum()
}

/// An `A` is mapped to a value of `1` here.
fn name_score(name: &str) -> u32 {
  name.chars().map(|c| c as u32 - 'A' as u32 + 1).sum()
}
