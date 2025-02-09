use crate::math::number_theory::lcm;

/// What is the smallest positive number that is evenly divisible by all of the numbers
/// from 1 to 20?
///
/// This question asks us to find the least common multiple of all numbers between 1 and 20.
/// Another way to think about it is to express each number as a product of prime powers. The lcm
/// will then be the product of multiplying the highest power of each prime number together.
pub fn solve() -> u32 {
  let numbers: Vec<u32> = (1..=20).collect();
  lcm_extended(&numbers)
}

fn lcm_extended(numbers: &[u32]) -> u32 {
  numbers.iter().fold(1, |acc, &n| lcm(acc, n))
}
