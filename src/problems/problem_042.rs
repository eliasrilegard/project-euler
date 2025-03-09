use std::fs;

/// How many words in the text file `resources/0042_words.txt` have a word score that is a triangle
/// number?
///
/// Solution is borderline identical to Problem 22, just with a different mapping.
pub fn solve() -> usize {
  let input = fs::read_to_string("resources/0042_words.txt").expect("Unable to read file");
  input
    .split(",")
    .map(|word| word.trim_matches('"'))
    .map(word_value)
    .filter(is_triangular)
    .count()
}

/// 'A' is mapped to 1
fn word_value(word: &str) -> u32 {
  word
    .to_ascii_lowercase()
    .chars()
    .map(|c| (c as u8 - b'a' + 1) as u32)
    .sum()
}

/// A number `n` is triangular if and only if `8n + 1` is a perfect square.
fn is_triangular(n: &u32) -> bool {
  let square = 8 * n + 1;
  let root = (square as f64).sqrt() as u32;
  root * root == square
}
