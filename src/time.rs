use crate::*;
use chrono::NaiveDateTime;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::*;

pub type DateTime = NaiveDateTime;
pub type StdDuration = std::time::Duration;
pub type ChronoDuration = chrono::Duration;

pub const S: Duration = Duration::in_s(1.0);
pub const MIN: Duration = Duration::in_s(60.0);
pub const HR: Duration = Duration::in_hours(1.0);
pub const DAY: Duration = Duration::in_hours(24.0);
pub const YR: Duration = Duration::in_days(365.25);

/// Elapsed game time in seconds.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct TimeFloat {
    pub value: Duration,
}

impl TimeFloat {
    #[inline]
    pub fn in_s(seconds: f64) -> Self {
        Self::new(seconds)
    }

    #[inline]
    pub fn in_days(days: f64) -> Self {
        Self::in_s(days * Duration::SECONDS_PER_DAY)
    }

    #[inline]
    fn new(value: f64) -> Self {
        Self {
            value: Duration::new(value),
        }
    }
}

impl Div for TimeFloat {
    type Output = f64;
    #[inline]
    fn div(self, rhs: Self) -> f64 {
        self.value / rhs.value
    }
}

impl Add<Duration> for TimeFloat {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Duration) -> Self {
        Self {
            value: self.value + rhs,
        }
    }
}

impl AddAssign<Duration> for TimeFloat {
    #[inline]
    fn add_assign(&mut self, rhs: Duration) {
        self.value += rhs;
    }
}

impl Sub<Duration> for TimeFloat {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Duration) -> Self {
        Self {
            value: self.value - rhs,
        }
    }
}

impl Sub for TimeFloat {
    type Output = Duration;
    #[inline]
    fn sub(self, rhs: Self) -> Duration {
        self.value - rhs.value
    }
}

impl Div<Duration> for TimeFloat {
    type Output = f64;
    #[inline]
    fn div(self, rhs: Duration) -> Self::Output {
        self.value / rhs
    }
}

impl Eq for TimeFloat {}

impl PartialOrd for TimeFloat {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for TimeFloat {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.partial_cmp(&other.value).unwrap()
    }
}

scalar! {
    struct Duration(f64) {
        fn in_s(seconds) -> Self;
    }
}

impl Duration {
    #[inline]
    pub const fn in_days(days: f64) -> Self {
        Self::in_s(days * Self::SECONDS_PER_DAY)
    }

    #[inline]
    pub const fn in_hours(hours: f64) -> Self {
        Self::in_s(hours * Self::SECONDS_PER_HOUR)
    }

    #[inline]
    pub fn days(&self) -> Days {
        Days(*self)
    }

    pub const SECONDS_PER_DAY: f64 = Self::SECONDS_PER_HOUR * 24.0;

    pub const SECONDS_PER_HOUR: f64 = 3600.0;

    pub const MAX: Duration = Duration::new(f64::MAX);
}

impl From<ChronoDuration> for Duration {
    #[inline]
    fn from(duration: ChronoDuration) -> Self {
        let seconds = duration.num_milliseconds() as f64 / 1e3;
        Duration::in_s(seconds)
    }
}

impl From<Duration> for ChronoDuration {
    #[inline]
    fn from(duration: Duration) -> Self {
        let microseconds = (duration.value * 1e6) as i64;
        ChronoDuration::microseconds(microseconds)
    }
}

impl From<Duration> for StdDuration {
    #[inline]
    fn from(duration: Duration) -> Self {
        let microseconds = (duration.value * 1e6) as u64;
        StdDuration::from_micros(microseconds)
    }
}

pub struct Days(Duration);

impl Display for Days {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let days = self.0 / Duration::in_days(1.0);
        write!(f, "{:.1} days", days)
    }
}

scalar! {
    struct DurationSquared(f64) {
        fn in_s2(s2) -> Self;
    }
}

scalar! {
    struct Frequency(f64) {
        fn per_s(s) -> Self;
    }
}

scalar_div! { f64 | Duration = Frequency }

scalar_squared!(Duration ^ 2 = DurationSquared);

scalar_div!(Length | Acceleration = DurationSquared);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn duration_float_from_duration() {
        let one_second = ChronoDuration::seconds(1);
        let one_second = Duration::from(one_second);

        assert_eq!(Duration::in_s(1.0), one_second);
    }
}
