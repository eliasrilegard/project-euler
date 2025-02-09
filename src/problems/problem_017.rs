/// If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many
/// letters would be used? Note! Do not count spaces or hyphens.
///
/// The hard part is dynamically converting all numbers to English words. Once that's done it's
/// simply a matter of counting how many letters there are in total.
pub fn solve() -> usize {
  (1..=1000).map(number_to_words).map(count_letters).sum()
}

/// Padding the first entry allows for natural indexing
const SINGLES: [&str; 20] = [
  "",
  "one",
  "two",
  "three",
  "four",
  "five",
  "six",
  "seven",
  "eight",
  "nine",
  "ten",
  "eleven",
  "twelve",
  "thirteen",
  "fourteen",
  "fifteen",
  "sixteen",
  "seventeen",
  "eighteen",
  "nineteen",
];

/// Padding the first entries allows more natural indexing
const TENS: [&str; 10] = [
  "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

/// Use recursion to handle combination logic
fn number_to_words(n: usize) -> String {
  match n {
    1..=19 => SINGLES[n].to_string(),

    20..=90 if n % 10 == 0 => TENS[n / 10].to_string(),

    21..=99 => {
      let tens = number_to_words(n / 10 * 10);
      let ones = number_to_words(n % 10);
      format!("{}-{}", tens, ones)
    }

    100..=999 => {
      let hundreds = number_to_words(n / 100) + " hundred";
      if n % 100 == 0 {
        hundreds
      } else {
        format!("{} and {}", hundreds, number_to_words(n % 100))
      }
    }

    1000 => "one thousand".to_string(),

    _ => unreachable!(),
  }
}

fn count_letters(s: String) -> usize {
  s.chars().filter(|c| c.is_alphabetic()).count()
}
