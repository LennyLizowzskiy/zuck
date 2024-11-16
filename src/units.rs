pub mod second {
    pub type Second = u64;

    pub const SECOND: Second = 1;

    /// Seconds per minute.
    pub const MINUTE: Second = SECOND * 60;

    /// Seconds per hour.
    pub const HOUR: Second = MINUTE * 60;

    /// Seconds per day.
    pub const DAY: Second = HOUR * 24;
    // pub const WEEK: Second = DAY * 7;

    /// Seconds per month.
    ///
    /// *calculated from [DAY] * 30.44*
    pub const MONTH: Second = 2_630_016;

    /// Seconds per year.
    ///
    /// *calculated from [DAY] * 365.25*
    pub const YEAR: Second = 31_557_600;
}

pub mod nanosecond {
    pub type Nanosecond = u128;

    pub const NANOSECOND: Nanosecond = 1;

    /// Nanoseconds per microsecond.
    pub const MICROSECOND: Nanosecond = NANOSECOND * 1000;

    /// Nanoseconds per millisecond.
    pub const MILLISECOND: Nanosecond = MICROSECOND * 1000;

    /// Nanoseconds per second.
    pub const SECOND: Nanosecond = MILLISECOND * 1000;

    /// Nanoseconds per minute.
    pub const MINUTE: Nanosecond = SECOND * 60;

    /// Nanoseconds per hour.
    pub const HOUR: Nanosecond = MINUTE * 60;

    /// Nanoseconds per day.
    pub const DAY: Nanosecond = HOUR * 24;

    /// Nanoseconds per month.
    ///
    /// *calculated from [DAY] * 30.44*
    pub const MONTH: Nanosecond = 2_630_016_000_000_000_000;

    /// Nanoseconds per year.
    ///
    /// *calculated from [DAY] * 365.25*
    pub const YEAR: Nanosecond = 31_556_736_000_000_000_000;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeUnit {
    Nanosecond,
    Microsecond,
    Millisecond,
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
}

impl core::fmt::Display for TimeUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TimeUnit::Nanosecond => "Nanosecond",
                TimeUnit::Microsecond => "Microsecond",
                TimeUnit::Millisecond => "Millisecond",
                TimeUnit::Second => "Second",
                TimeUnit::Minute => "Minute",
                TimeUnit::Hour => "Hour",
                TimeUnit::Day => "Day",
                TimeUnit::Week => "Week",
                TimeUnit::Month => "Month",
                TimeUnit::Year => "Year",
            }
        )
    }
}

impl core::str::FromStr for TimeUnit {
    type Err = error::Error;

    #[rustfmt::skip] // so match won't be formatted
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use error::Error::*;

        match s {
            "ns" | "nsec" | "nsecs" | "nanosec" | "nanosecs" | "nanosecond" | "nanoseconds" => Ok(TimeUnit::Nanosecond),
            "μs" | "us" | "usec" | "usecs" | "microsec" | "microsecs" | "microsecond" | "microseconds" => Ok(TimeUnit::Microsecond),
            "ms" | "msec" | "msecs" | "millisecond" | "milliseconds" => Ok(TimeUnit::Millisecond),
            "s" | "sec" | "secs" | "second" | "seconds" => Ok(TimeUnit::Second),
            "m" | "min" | "mins" | "minute" | "minutes" => Ok(TimeUnit::Minute),
            "h" | "hr" | "hrs" | "hour" | "hours" => Ok(TimeUnit::Hour),
            "d" | "day" | "days" => Ok(TimeUnit::Day),
            "w" | "wk" | "wks" | "week" | "weeks" => Ok(TimeUnit::Week),
            "mo" | "month" | "months" => Ok(TimeUnit::Month),
            "y" | "yr" | "yrs" | "year" | "years" => Ok(TimeUnit::Year),

            _ => Err(UnknownUnit),
        }
    }
}

pub mod error {
    #[derive(Debug, PartialEq, Eq)]
    pub enum Error {
        UnknownUnit,
    }
}
