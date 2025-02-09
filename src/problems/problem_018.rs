/// Find the maximum total from top to bottom of the triangle below.
///
/// While this input is small enough to be brute-forced, it also happens to smell like dynamic
/// programming from miles away. Functions are made public to be used in Problem 67 as well.
pub fn solve() -> u32 {
  let mut triangle = parse_triangle(TRIANGLE);
  solve_triangle(&mut triangle)
}

const TRIANGLE: &str = "
75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23
";

pub fn parse_triangle(input: &str) -> Vec<Vec<u32>> {
  input
    .trim()
    .lines()
    .map(|line| {
      line
        .split_whitespace()
        .map(|num| num.parse::<u32>().expect("Invalid number"))
        .collect()
    })
    .collect()
}

/// Bottom-up dynamic programming approach. We start in the 2nd to bottom row, adding to each entry
/// the higher of the numbers below it. Work upwards and we have our solution at the top.
pub fn solve_triangle(triangle: &mut [Vec<u32>]) -> u32 {
  for row in (0..triangle.len() - 1).rev() {
    for col in 0..triangle[row].len() {
      triangle[row][col] += triangle[row + 1][col].max(triangle[row + 1][col + 1]);
    }
  }
  triangle[0][0]
}
