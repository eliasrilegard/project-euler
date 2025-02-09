/// Which starting number, under one million, produces the longest Collatz sequence? Note, once the
/// chain starts, the terms are allowed to go above one million.
///
/// Use memoization to store already computed lengths. This approach *should* be equally fast using
/// a HashMap for caching, however the "constant" lookup is expensive enough that we're better off
/// just using a vanilla array instead.
pub fn solve() -> u32 {
  const LIMIT: usize = 1_000_000;
  let mut cache = vec![0; LIMIT];

  let mut max_length = 0;
  let mut result = 0;

  for i in 1..LIMIT {
    let length = collatz_length(i as u64, &mut cache);
    if length > max_length {
      max_length = length;
      result = i as u32;
    }
  }

  result
}

/// Compute the length of a Collatz sequence with a given start, sped up by memoization.
fn collatz_length(mut num: u64, cache: &mut [u32]) -> u32 {
  if is_cached(num, cache) {
    return cache[num as usize];
  }

  let start = num as usize;
  let mut count = 0;

  while num != 1 && !is_cached(num, cache) {
    count += 1;
    num = if num % 2 == 0 { num / 2 } else { 3 * num + 1 };
  }

  let total_length = count + cache[num as usize];
  cache[start] = total_length;

  total_length
}

fn is_cached(num: u64, cache: &[u32]) -> bool {
  num < cache.len() as u64 && cache[num as usize] != 0
}
