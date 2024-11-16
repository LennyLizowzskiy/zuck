use core::time::Duration as RDuration;

use crate::{
    formatter::FormatterOptions,
    units::{
        nanosecond::{self as ns, Nanosecond},
        second::{self as s, Second},
    },
    util::{checkedu128::CheckedU128, checkedu64::CheckedU64},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Duration {
    pub nanoseconds: u64,
    pub microseconds: u64,
    pub milliseconds: u64,
    pub seconds: u64,
    pub minutes: u64,
    pub hours: u64,
    pub days: u64,
    pub months: u64,
    pub years: u64,
}

pub mod error {
    #[derive(Debug, PartialEq, Clone)]
    pub enum Error {
        IntOverflow,
    }
}

impl Duration {
    /// Normalizes the time units within the `Duration` struct to ensure that each unit
    /// is within its typical range. For example, it ensures that there are less than 1000 nanoseconds
    /// in a microsecond, less than 1000 microseconds in a millisecond, and so on.
    pub fn normalize(mut self) -> Self {
        if self.nanoseconds >= 1000 {
            self.microseconds += self.nanoseconds / 1000;
            self.nanoseconds = self.nanoseconds % 1000;
        }

        if self.microseconds >= 1000 {
            self.milliseconds += self.microseconds / 1000;
            self.microseconds = self.microseconds % 1000;
        }

        if self.milliseconds >= 1000 {
            self.seconds += self.milliseconds / 1000;
            self.milliseconds = self.milliseconds % 1000;
        }

        if self.seconds >= 60 {
            self.minutes += self.seconds / 60;
            self.seconds = self.seconds % 60;
        }

        if self.minutes >= 60 {
            self.hours += self.minutes / 60;
            self.minutes = self.minutes % 60;
        }

        if self.hours >= 24 {
            self.days += self.hours / 24;
            self.hours = self.hours % 24;
        }

        if self.days >= 30 {
            self.months += self.days / 30;
            self.days = self.days % 30;
        }

        if self.months >= 12 {
            self.years += self.months / 12;
            self.months = self.months % 12;
        }

        self
    }

    /// Converts the duration into nanoseconds without checking for overflow.
    pub fn into_nanoseconds_unchecked(&self) -> Nanosecond {
        (self.nanoseconds as u128)
            + (self.microseconds as u128 * ns::MICROSECOND)
            + (self.milliseconds as u128 * ns::MILLISECOND)
            + (self.seconds as u128 * ns::SECOND)
            + (self.minutes as u128 * ns::MINUTE)
            + (self.hours as u128 * ns::HOUR)
            + (self.days as u128 * ns::DAY)
            + (self.months as u128 * ns::MONTH)
            + (self.years as u128 * ns::YEAR)
    }

    /// Converts the duration into nanoseconds with overflow checking.
    pub fn into_nanoseconds(&self) -> Result<Nanosecond, error::Error> {
        CheckedU128::from(self.nanoseconds as u128)
            .add_mul_result(self.microseconds.into(), ns::MICROSECOND)
            .add_mul_result(self.milliseconds.into(), ns::MILLISECOND)
            .add_mul_result(self.seconds.into(), ns::SECOND)
            .add_mul_result(self.minutes.into(), ns::MINUTE)
            .add_mul_result(self.hours.into(), ns::HOUR)
            .add_mul_result(self.days.into(), ns::DAY)
            .add_mul_result(self.months.into(), ns::MONTH)
            .add_mul_result(self.years.into(), ns::YEAR)
            .ok_or(error::Error::IntOverflow)
    }

    /// Converts the duration into seconds without checking for overflow.
    pub fn into_seconds_unchecked(&self) -> Second {
        (self.nanoseconds / 1_000_000_000)
            + (self.microseconds / 1_000_000)
            + (self.milliseconds / 1_000)
            + self.seconds
            + (self.minutes * s::MINUTE)
            + (self.hours * s::HOUR)
            + (self.days * s::DAY)
            + (self.months * s::MONTH)
            + (self.years * s::YEAR)
    }

    /// Converts the duration into seconds with overflow checking.
    pub fn into_seconds(&self) -> Result<Second, error::Error> {
        CheckedU64::from(
            (self.nanoseconds / 1_000_000_000)
                + (self.microseconds / 1_000_000)
                + (self.milliseconds / 1_000)
                + self.seconds,
        )
        .add_mul_result(self.minutes, s::MINUTE)
        .add_mul_result(self.hours, s::HOUR)
        .add_mul_result(self.days, s::DAY)
        .add_mul_result(self.months, s::MONTH)
        .add_mul_result(self.years, s::YEAR)
        .ok_or(error::Error::IntOverflow)
    }

    pub fn from_seconds(s: Second) -> Self {
        // remaining seconds to divide
        let mut s = s;

        let years = s / s::YEAR;
        s = s % s::YEAR;

        let months = s / s::MONTH;
        s = s % s::MONTH;

        let days = s / s::DAY;
        s = s % s::DAY;

        let hours = s / s::HOUR;
        s = s % s::HOUR;

        let minutes = s / s::MINUTE;
        s = s % s::MINUTE;

        Self {
            seconds: s,
            minutes,
            hours,
            days,
            months,
            years,
            ..Default::default()
        }
    }

    pub fn from_nanoseconds(ns: Nanosecond) -> Self {
        // remaining ns to divide
        let mut ns = ns;

        let years = ns / ns::YEAR;
        ns = ns % ns::YEAR;

        let months = ns / ns::MONTH;
        ns = ns % ns::MONTH;

        let days = ns / ns::DAY;
        ns = ns % ns::DAY;

        let hours = ns / ns::HOUR;
        ns = ns % ns::HOUR;

        let minutes = ns / ns::MINUTE;
        ns = ns % ns::MINUTE;

        let seconds = ns / ns::SECOND;
        ns = ns % ns::SECOND;

        let milliseconds = ns / ns::MILLISECOND;
        ns = ns % ns::MILLISECOND;

        let microseconds = ns / ns::MICROSECOND;
        ns = ns % ns::MICROSECOND;

        // let nanoseconds = remaining ns

        Self {
            nanoseconds: ns as u64,
            microseconds: microseconds as u64,
            milliseconds: milliseconds as u64,
            seconds: seconds as u64,
            minutes: minutes as u64,
            hours: hours as u64,
            days: days as u64,
            months: months as u64,
            years: years as u64,
        }
    }
}

impl core::fmt::Display for Duration {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.format(&FormatterOptions::default()))
    }
}

impl Duration {
    pub fn from_rs_duration_as_secs(value: RDuration) -> Result<Self, core::num::TryFromIntError> {
        Ok(Duration::from_seconds(value.as_secs()))
    }

    pub fn from_rs_duration_as_nanos(value: RDuration) -> Result<Self, core::num::TryFromIntError> {
        Ok(Duration::from_nanoseconds(value.as_nanos()))
    }
}

impl core::convert::TryFrom<RDuration> for Duration {
    type Error = core::num::TryFromIntError;

    fn try_from(value: RDuration) -> Result<Self, Self::Error> {
        Ok(Duration::from_rs_duration_as_nanos(value)?)
    }
}

impl TryInto<RDuration> for Duration {
    type Error = error::Error;

    fn try_into(self) -> Result<RDuration, Self::Error> {
        let rdur = self.into_nanoseconds()?;
        Ok(RDuration::from_nanos(rdur as _))
    }
}

#[cfg(test)]
mod test {
    use core::{str::FromStr, u64};

    use crate::{units, Duration};

    #[test]
    fn from_eq_into_seconds() {
        let orig_raw = 60000000 as units::second::Second;
        let orig = Duration::from_seconds(orig_raw);
        let converted_back = orig.into_seconds().unwrap();

        assert_eq!(orig_raw, converted_back);
    }

    #[test]
    fn from_eq_into_nanoseconds() {
        let orig_raw = 6000000000 as units::nanosecond::Nanosecond;
        let orig = Duration::from_nanoseconds(orig_raw);
        let converted_back = orig.into_nanoseconds().unwrap();

        assert_eq!(orig_raw, converted_back);
    }

    #[test]
    fn normalize_normal() {
        let orig = Duration::from_str("28mo35d49h200m150s50020ms").expect("fail on valid input");
        let balanced = orig.clone().normalize();
        assert_ne!(orig, balanced); // orig value != balanced value in this case

        assert_eq!(
            balanced,
            Duration {
                nanoseconds: 0,
                microseconds: 0,
                milliseconds: 20,
                seconds: 20,
                minutes: 23,
                hours: 4,
                days: 7,
                months: 5,
                years: 2
            }
        );
    }

    #[test]
    fn normalize_nothing_to_balance() {
        let result = Duration::from_str("2d50s")
            .expect("fail on valid input")
            .normalize();
        let expected = Duration {
            days: 2,
            seconds: 50,
            ..Default::default()
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn into_seconds() {
        let d = Duration::from_str("3d").expect("fail on valid input");
        let result = d.into_seconds().expect("fail on valid int");
        let expected = 259200 as units::second::Second;

        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic(expected = "IntOverflow")]
    fn into_seconds_overflow() {
        Duration::from_str("99999999999999y")
            .expect("fail on valid input")
            .into_seconds()
            .unwrap();
    }

    #[test]
    fn into_nanoseconds() {
        let d = Duration::from_str("1m").expect("fail on valid input");
        let result = d.into_nanoseconds().expect("fail on valid int");
        let expected = 60000000000 as units::nanosecond::Nanosecond;

        assert_eq!(result, expected);
    }

    #[test]
    fn from_nanoseconds() {
        // todo
    }

    #[test]
    fn from_seconds() {
        let result = Duration::from_seconds(2000000);
        let expected = Duration {
            nanoseconds: 0,
            microseconds: 0,
            milliseconds: 0,
            seconds: 20,
            minutes: 33,
            hours: 3,
            days: 23,
            months: 0,
            years: 0,
        };
        assert_eq!(2000000, expected.into_seconds().unwrap());

        assert_eq!(result, expected);
    }

    #[test]
    fn from_seconds_large() {
        let result = Duration::from_seconds(200000000);
        let expected = Duration {
            nanoseconds: 0,
            microseconds: 0,
            milliseconds: 0,
            seconds: 56,
            minutes: 18,
            hours: 13,
            days: 1,
            months: 4,
            years: 6,
        };
        assert_eq!(200000000, expected.into_seconds().unwrap());

        assert_eq!(result, expected);
    }

    #[test]
    fn from_rust_duration() {
        use core::time::Duration as RDuration;

        let rdur = RDuration::from_secs(6000);
        let result = Duration::try_from(rdur).unwrap();

        // todo
    }
}
