/// How many Sundays fell on the first of the month during the twentieth century (1901-01-01 to
/// 2000-12-31)?
///
/// We are given that 1900-01-01 was a Monday, so let's assign Mondays the numerical value `0`. If
/// we step forward the year of 1900, we arrive at 1901-01-01, which is our starting point. Loop
/// through every month, checking if it starts on a Sunday (`6`), then "pass" the month by adding
/// the number of days in the month to arrive at the start of the next. This approach keeps
/// `day_of_week` in the range `0` to `6` (inclusive) to prevent overflow at ridiculous scales.
pub fn solve() -> u32 {
  let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

  #[allow(clippy::identity_op)]
  let mut day_of_week = (0 + days_in_year(1900)) % 7; // 1 Jan 1900 was a Monday
  let mut sundays = 0;

  for year in 1901..=2000 {
    for days in days_in_month {
      if day_of_week == 6 {
        sundays += 1;
      }
      day_of_week += days + if days == 28 && is_leap_year(year) { 1 } else { 0 };
      day_of_week %= 7;
    }
  }

  sundays
}

fn is_leap_year(year: u32) -> bool {
  (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_year(year: u32) -> u32 {
  if is_leap_year(year) {
    366
  } else {
    365
  }
}
