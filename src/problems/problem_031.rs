/// How many different ways can £2 be made using any combination of the coins 1p, 2p, 5p, 10p, 20p,
/// 50p, £1 (100p), and £2 (200p)?
///
/// This problem is well suited for a dynamic programming solution. There's only one way to make 0p,
/// i.e. by using nothing. For every coin, update the number of ways to form every sum (above said
/// coin value) by adding on the number of ways to form the sum of (coin) value less:
/// `ways[i] := ways[i] + ways[i - coin]`, meaning the number of ways to make the sum `i` is the
/// number of ways to make `i` *without* using the current coin, plus all the ways of making
/// `i - coin` and then adding on the coin.
pub fn solve() -> u32 {
  let target = 200;
  let coins = [1, 2, 5, 10, 20, 50, 100, 200];

  let mut ways = vec![0; target + 1];
  ways[0] = 1; // One way to reach zero pence

  for coin in coins {
    for i in coin..=target {
      ways[i] += ways[i - coin];
    }
  }

  ways[target]
}
