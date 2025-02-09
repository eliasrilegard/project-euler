/// Check whether a number is palindrome, i.e. the same when written backwards as forwards.
pub fn is_palindrome(n: u32) -> bool {
  let s = n.to_string();
  s.chars().eq(s.chars().rev())
}
